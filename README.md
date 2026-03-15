# kvstore

A lightweight key-value store built in Rust. Accepts commands over a TCP connection, stores data in memory, and persists to disk as JSON.

## Commands

| Command | Description | Example |
|---------|-------------|---------|
| `SET key value` | Store a value | `SET name daniel` |
| `GET key` | Retrieve a value | `GET name` |
| `DEL key` | Delete a key | `DEL name` |

## Getting started

**With Rust**

```bash
git clone https://github.com/danielbusnz-lgtm/kvstore
cd kvstore
cargo run
```

**With Docker**

```bash
docker build -t kvstore .
docker run -p 6379:6379 kvstore
```

The server starts on `127.0.0.1:6379`.

## Connecting

Use any TCP client to connect and send commands:

```powershell
$client = New-Object System.Net.Sockets.TcpClient("127.0.0.1", 6379)
$stream = $client.GetStream()
$writer = New-Object System.IO.StreamWriter($stream)
$reader = New-Object System.IO.StreamReader($stream)
$writer.WriteLine("SET name daniel")
$writer.Flush()
$reader.ReadLine()  # OK
$writer.WriteLine("GET name")
$writer.Flush()
$reader.ReadLine()  # daniel
$client.Close()
```

## Persistence

Data is saved to `store.json` on every write and loaded back on startup. Stopping and restarting the server preserves all data.

## Stack

- Rust
- Tokio — async runtime
- Serde — JSON serialization
