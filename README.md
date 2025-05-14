
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

### 1. **Add to Your `Cargo.toml`**
```toml
[dependencies]
starlink_client = "0.1.0"
```

> Or reference from your local path or GitHub.

---

### 2. **Example Usage**
```rust
use starlink_client::client::DishClient;

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

> ✅ See full example in [examples/usage.rs](examples/usage.rs)

---

## 🛠️ Features Roadmap

- [ ] Streaming support
- [ ] Configurable polling API
- [ ] Prometheus / Grafana integration
- [ ] CLI binary interface
- [ ] JSON serialization helpers

---

## 📦 Building From Source

Clone the repository and build:

```bash
git clone https://github.com/your-org/starlink-grpc-client.git
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
