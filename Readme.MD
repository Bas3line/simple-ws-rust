# 🔗 WebSocket Echo Server (Axum)

A minimal **WebSocket Echo Server** built with [Axum](https://github.com/tokio-rs/axum) in **Rust**.

---

## Usage

```sh
cargo run
```

Connect to: **ws\://127.0.0.1:3000/ws**

---

## Tech Stack

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![WebSocket](https://img.shields.io/badge/WebSocket-010101?style=for-the-badge&logo=socket.io&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-222222?style=for-the-badge&logo=tokio&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-3566E0?style=for-the-badge&logo=rust&logoColor=white)

---

## 📦 Dependencies

```toml
[dependencies]
axum = { version = "0.7", features = ["ws"] }
tokio = { version = "1", features = ["full"] }
futures-util = { version = "0.3", features = ["sink"] }
```
