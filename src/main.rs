use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

type Store = Arc<Mutex<HashMap<String, String>>>;

fn load_from_disk() -> HashMap<String, String> {
    match std::fs::read_to_string("store.json") {
        Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
        Err(_) => HashMap::new(),
    }
}

fn save_to_disk(store: &HashMap<String, String>) {
    let json = serde_json::to_string(store).unwrap();
    std::fs::write("store.json", json).unwrap();
}

#[tokio::main]
async fn main() {
    let store: Store = Arc::new(Mutex::new(load_from_disk()));
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening on port 6379");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        let store = Arc::clone(&store);
        tokio::spawn(async move {
            handle_connection(socket, store).await;
        });
    }
}

async fn handle_connection(
    mut socket: tokio::net::TcpStream,
    store: Store,
) {
    let mut buf = [0; 1024];
    loop {
        let n = socket.read(&mut buf).await.unwrap_or(0);
        if n == 0 {
            return;
        }
        let input = String::from_utf8_lossy(&buf[..n]);
        let response = handle_command(input.trim(), &store);
        socket.write_all(response.as_bytes()).await.unwrap();
    }
}

fn handle_command(input: &str, store: &Store) -> String {
    let parts: Vec<&str> = input.splitn(3, ' ').collect();
    match parts.as_slice() {
        ["GET", key] => {
            let store = store.lock().unwrap();
            match store.get(*key) {
                Some(val) => format!("{}\n", val),
                None => "nil\n".to_string(),
            }
        }
        ["SET", key, value] => {
            let mut store = store.lock().unwrap();
            store.insert(key.to_string(), value.to_string());
            save_to_disk(&store);
            "OK\n".to_string()
        }
        ["DEL", key] => {
            let mut store = store.lock().unwrap();
            store.remove(*key);
            save_to_disk(&store);
            "OK\n".to_string()
        }
        _ => "ERR unknown command\n".to_string(),
    }
}
