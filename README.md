## HTTPs Server

(Currently) Install via Rust:
```
git clone https://github.com/lambdastackio/httpd.git
# These two are referred from Cargo.toml as x = { path = "../x" }:
git clone https://github.com/lambdastackio/tokio-http2.git
git clone https://github.com/abonander/multipart.git
cd httpd
cargo update
cargo run help
```

This server supports HTTP(s).
