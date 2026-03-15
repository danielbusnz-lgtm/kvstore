# kvstore

A lightweight key-value store built in Rust. Accepts commands over a TCP connection, stores data in memory, and persists to disk as JSON. Includes a terminal UI for browsing and managing keys interactively.

## Commands

| Command | Description | Example |
|---------|-------------|---------|
| `SET key value` | Store a value | `SET name daniel` |
| `GET key` | Retrieve a value | `GET name` |
| `DEL key` | Delete a key | `DEL name` |
| `KEYS` | List all keys | `KEYS` |

## Getting started

**With Rust**

```bash
git clone https://github.com/danielbusnz-lgtm/kvstore
cd kvstore
cargo run --bin kvstore
```

**With Docker**

```bash
docker build -t kvstore .
docker run -p 6379:6379 kvstore
```

The server starts on `127.0.0.1:6379`.

## Terminal UI

Run the TUI client in a second terminal while the server is running:

```bash
cargo run --bin tui
```

- Arrow keys to navigate keys
- Selected key's value is shown in the right panel
- Type commands in the input box and hit Enter
- Press Esc to exit

## Connecting manually

Use any TCP client to send commands directly:

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
- Ratatui — terminal UI
- Crossterm — terminal input and rendering
