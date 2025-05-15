# Starlink gRPC Client

A **safe**, **async**, and **type-safe** Rust client for querying status information from a **Starlink Gen 3 Dishy** via **gRPC**.

---

## ✨ Features

- ✅ Simple API to query **dish status**
- ✅ Fully **async** with [Tokio](https://crates.io/crates/tokio)
- ✅ **Type-safe models** wrapping raw gRPC responses
- ✅ Clean **error handling** with `thiserror`
- ✅ Ready for **polling integrations** or **CLI tools**

---

## 🚀 Getting Started

### 1. **Install Protobuf Compiler**

This library requires `protoc` (Protocol Buffers Compiler) to be installed.

- **macOS**:  
  ```bash
  brew install protobuf
  ```
- **Linux**:  
  Install via package manager or [official releases](https://github.com/protocolbuffers/protobuf/releases).

- **Windows**:  
  Download from [https://github.com/protocolbuffers/protobuf/releases](https://github.com/protocolbuffers/protobuf/releases).

Or set the `PROTOC` environment variable if you have a custom installation.

---

### 2. **Add to Your `Cargo.toml`**

```toml
[dependencies]
starlink-grpc-client = "0.3.1"
```

---

### 3. **Example Usage**

```rust
use starlink_grpc_client::client::DishClient;

#[tokio::main]
async fn main() {
    let mut client = DishClient::connect("http://dishy.starlink.com:9200")
        .await
        .expect("Failed to connect to Dish");

    let status = client.get_status()
        .await
        .expect("Failed to fetch dish status");

    println!("{:#?}", status);
}
```

> ✅ See working examples in [examples/usage.rs](examples/usage.rs), [examples/stream.rs](examples/stream.rs) and [examples/stream_verbose.rs](examples/stream_verbose.rs)

---

## 📑 Public API Summary

| Method                   | Signature                                                                                                                                                         | Description                                        |
|--------------------------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------|
| **connect**              | `pub async fn connect(endpoint: &str) -> Result<Self, DishError>`                                                                                                 | Create a new client by dialing the given URL.      |
| **get_status**           | `pub async fn get_status(&mut self) -> Result<DishStatus, DishError>`                                                                                             | Perform a single, unary status RPC.                |
| **stream_status**        | `pub async fn stream_status(&mut self) -> Result<impl Stream<Item = Result<DishStatus, DishError>>, DishError>`                                                   | Poll `getStatus` once per second, silently.        |
| **stream_status_logged** | `pub async fn stream_status_logged(&mut self) -> Result<impl Stream<Item = Result<(DishStatus, Duration), DishError>>, DishError>`                               | Poll with outbound logs and true RTT measurement.  |

---

## 🛠️ Features Roadmap

- [ ] Push-based streaming support (server-driven updates)
- [ ] Configurable polling API
- [ ] Prometheus / Grafana integration
- [ ] CLI binary interface
- [ ] JSON serialization helpers

---

## 📦 Building From Source

Clone the repository and build:

```bash
git clone https://github.com/andywwright/starlink-grpc-client.git
cd starlink-grpc-client
cargo build
```

---

## ✅ Semantic Versioning

This project follows **[Semantic Versioning 2.0.0](https://semver.org/)**:

- **MAJOR**: Breaking changes
- **MINOR**: Backward-compatible features
- **PATCH**: Bug fixes

_Current Version: **0.1.0** (Developer Preview)_

---

## 📝 License

MIT License. See [LICENSE](LICENSE) for details.

---

## 💬 Feedback and Contributions

- **Issues** and **PRs** welcome!
- Please follow [Rust API Design Guidelines](https://rust-lang.github.io/api-guidelines/).