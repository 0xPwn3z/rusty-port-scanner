# Rusty Port Scanner

A fast, asynchronous TCP port scanner written in Rust.

`rusty-port-scanner` helps you quickly check which TCP ports are open on a target host by scanning a single port or a range of ports with configurable timeout and concurrency.

> Warning: Use this tool only on systems and networks you own or are explicitly authorized to test.

## Features

- Asynchronous TCP scanning with `tokio`
- Configurable scan range (`start-port` to `end-port`)
- Adjustable timeout per connection attempt
- Adjustable maximum concurrency using a semaphore
- Simple CLI powered by `clap`

## Requirements

- Rust toolchain (stable, recent)
- Cargo (included with Rust)

Install Rust if needed:

```powershell
winget install Rustlang.Rustup
```

## Installation

### Option 1: Run from source

```powershell
git clone https://github.com/0xPwn3z/rusty-port-scanner.git
Set-Location rusty-port-scanner
cargo run -- --help
```

### Option 2: Build a release binary

```powershell
Set-Location rusty-port-scanner
cargo build --release
```

Binary location:

- Windows: `target\release\rusty-port-scanner.exe`
- Linux/macOS: `target/release/rusty-port-scanner`

### Option 3: Install locally with Cargo

```powershell
Set-Location rusty-port-scanner
cargo install --path .
```

Then run:

```powershell
rusty-port-scanner --help
```

## Usage

```text
rusty-port-scanner [OPTIONS] --address <ADDRESS> --start-port <START_PORT>
```

### CLI Options

| Option | Type | Required | Default | Description |
| --- | --- | --- | --- | --- |
| `-a, --address <ADDRESS>` | string | Yes | - | Target IP or hostname |
| `-s, --start-port <START_PORT>` | u16 | Yes | - | First port to scan |
| `-e, --end-port <END_PORT>` | u16 | No | `start-port` | Last port to scan |
| `-t, --timeout <TIMEOUT>` | u64 | No | `5` | Timeout in seconds per connection |
| `-c, --concurrency <CONCURRENCY>` | usize | No | `30` | Maximum concurrent connection attempts |
| `-h, --help` | flag | No | - | Show help |

## Examples

### Scan a single port

```powershell
rusty-port-scanner --address 127.0.0.1 --start-port 22
```

### Scan a port range

```powershell
rusty-port-scanner --address scanme.nmap.org --start-port 1 --end-port 1024
```

### Increase speed with more concurrency

```powershell
rusty-port-scanner --address 192.168.1.10 --start-port 1 --end-port 1000 --concurrency 200
```

### Use a shorter timeout

```powershell
rusty-port-scanner --address 10.0.0.5 --start-port 1 --end-port 65535 --timeout 2
```

## How It Works

The scanner:

1. Parses CLI arguments with `clap`.
2. Iterates over the requested port range.
3. Spawns asynchronous tasks with `tokio`.
4. Limits active tasks using a `Semaphore` (`--concurrency`).
5. Tries a TCP connection per port with `tokio::time::timeout`.
6. Prints open ports and exits when all tasks complete.

## Limitations

- Scans only TCP ports.
- Performs connect scans (not SYN/half-open scans).
- Output is minimal and not exported to file formats (JSON/CSV).
- Very high concurrency can cause local resource pressure (sockets/CPU).
- Network firewalls and IDS/IPS may throttle or block results.

## Security and Ethics

Port scanning can be interpreted as hostile behavior if unauthorized.

- Scan only assets you own or are permitted to test.
- Prefer controlled lab environments.
- Follow local laws, organizational policies, and rules of engagement.

## Development

### Project structure

- `src/main.rs`: CLI parsing and scan logic
- `Cargo.toml`: package metadata and dependencies

### Build and check

```powershell
cargo fmt
cargo clippy -- -D warnings
cargo build
cargo test
```

## Troubleshooting

- `connection timed out`: increase `--timeout`, reduce `--concurrency`, verify network path.
- `no open ports found`: confirm target host is reachable and services are listening.
- DNS/hostname issues: test with direct IP to isolate name resolution problems.
- Permission or endpoint protection issues: run in an allowed environment and review local security controls.

## License

This project is licensed under the MIT License.
See `LICENSE` for full text.

## Author

Copyright (c) 2026 `0xPwn3z`
