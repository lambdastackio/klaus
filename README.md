## HTTPs Server

(Currently) Install via Rust:
>Install Rust

>Git clone this repo

>cargo build

This server supports HTTP(s).

#### Setting up on Linux (RHEL/CentOS/AWS AMI)
1. sudo yum groupinstall -y 'Development Tools'
2. sudo yum install -y git
3. sudo yum install -y openssl-devel
4. curl https://sh.rustup.rs -sSf | sh
5. source $HOME/.cargo/env
6. rustup update nightly
7. rustup default nightly

Then just git clone this repo, tokio-http2 and maybe multipart (this one may find a new version with no default hyper)
Build tokio-http2 first and then this repo

Note: May need to `cargo update` first
