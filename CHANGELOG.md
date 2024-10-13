## 1.0.1 (2024-10-13)

### Fix

- **Named-Pipe-Server/new/Server-Options/Pipe-Mode**: switch from messages to bytes

## 1.0.0 (2024-10-13)

### BREAKING CHANGE

- Switching to Tokio for implementation of the main function
- New Rust Base Project has been added

### Feat

- **main/Name-Pipe-to-TCP-Stream-Proxy**: implement
- **main/Command-Line-Argument-Parser**: integrate
- **main/Splice::Readable**: implement trait for tcp stream and named pipe server
- **Splice**: add
- **Named-Pipe-Server-Manager**: add
- **src/main**: switch to tokio main
