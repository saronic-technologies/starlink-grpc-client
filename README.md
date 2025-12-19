# Starlink gRPC Client

A **safe**, **async**, and **type-safe** Rust client for querying status information from a **Starlink Gen 3 Dishy** via **gRPC**


- API version: **33**
- Fully tested on hardware version: **rev4_panda_prod1**

---

## ✨ Features

- ✅ Simple API to query **dish status**
- ✅ Fully **async** with [Tokio](https://crates.io/crates/tokio)
- ✅ **Type-safe models** wrapping raw gRPC responses
- ✅ Clean **error handling** with `thiserror`
- ✅ Ready for **polling integrations** or **CLI tools**

---

## 🚀 Getting Started

### **Add to Your `Cargo.toml`**

```toml
[dependencies]
starlink-grpc-client = "0.4.1"
tokio = "1.45.0"
```

(or whatever the last version is)

---

### **Example Usage**

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

> ✅ See working examples in [usage.rs](examples/usage.rs), [stream.rs](examples/stream.rs) and with simple [charts](https://github.com/andywwright/starlink-web-dashboard)

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

## 🧱 Regenerating gRPC Bindings (Protobuf)

If you change the `.proto` files and need to regenerate the Rust bindings, use the **build-protos** feature:

```bash
cargo build --features build-protos
```

- This will regenerate files into `proto_bindings`.
- You **must commit** these regenerated files if you want the default build to work for consumers.

**When to use this:**
- When updating `.proto` definitions.
- When preparing a new crate release with updated API.

**When _not_ needed:**
- Regular consumers or users **do not need to run this**.
- Default builds use the already checked-in generated code without `protoc`.

### Using System Protoc (Nix, Bazel, etc.)

By default, the `build-protos` feature uses a vendored `protoc` binary. If you're using Nix, Bazel, or another build system that prefers system dependencies, you can use your own `protoc`:

**Option 1: Use system protoc without vendored binary**
```bash
# Ensure protoc is installed on your system
# On NixOS, add protobuf to your buildInputs

# Build with build-protos but without vendored protoc
cargo build --features build-protos --no-default-features
```

**Option 2: Set PROTOC environment variable**
```bash
export PROTOC=/path/to/your/protoc
cargo build --features build-protos
```

**For Nix users:**
The vendored protoc is now optional, so crate2nix builds will work correctly. The `build-protos` feature automatically enables `vendored-protoc`, but you can disable it if needed.

---

## ✅ Semantic Versioning

This project follows **[Semantic Versioning 2.0.0](https://semver.org/)**:

- **MAJOR**: Breaking changes
- **MINOR**: Backward-compatible features
- **PATCH**: Bug fixes

---

## 📝 License

MIT License. See [LICENSE](LICENSE) for details.

---

## 💬 Feedback and Contributions

- **Issues** and **PRs** welcome!
- Please follow [Rust API Design Guidelines](https://rust-lang.github.io/api-guidelines/).