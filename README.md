## Klaus
Powerful, fast and safe time saving server. API, Agent, HTTP(s), HTTP/2, RPC based server that can be used in any micro-services architecture.

Based on Futures (promises), Async I/O that provides a very simple interface for application (server) developers while abstracting away the complex multi-threaded Futures Async I/O layers.

(Currently) Install via Rust:
#### Setting up on OSX
```
git clone https://github.com/lambdastackio/httpd.git
# These two are referred from Cargo.toml as x = { path = "../x" }:
git clone https://github.com/lambdastackio/tokio-http2.git
git clone https://github.com/abonander/multipart.git
cd httpd
cargo update
cargo run help
```

#### Setting up on Linux (RHEL/CentOS/AWS AMI)
```
1. sudo yum groupinstall -y 'Development Tools'
2. sudo yum install -y git
3. sudo yum install -y openssl-devel
4. curl https://sh.rustup.rs -sSf | sh
5. source $HOME/.cargo/env
6. rustup update nightly
7. rustup default nightly
```
Then just git clone this repo, tokio-http2 and maybe multipart (this one may find a new version with no default hyper)
Build tokio-http2 first and then this repo

Note: May need to `cargo update` first
