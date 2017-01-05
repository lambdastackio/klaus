// Copyright 2016 LambdaStack All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]

// NOTE: This attribute only needs to be set once.
#![doc(html_logo_url = "https://lambdastackio.github.io/static/images/lambdastack-200x200.png",
       html_favicon_url = "https://lambdastackio.github.io/static/images/favicon.ico",
       html_root_url = "https://lambdastackio.github.io/lsiohttps/lsiohttps/index.html")]

extern crate futures;
extern crate num_cpus;
extern crate pretty_env_logger;
extern crate url;
//extern crate filetime;
extern crate toml;
extern crate rustc_serialize;
extern crate multipart;
#[macro_use] extern crate log;
#[macro_use] extern crate clap;
#[macro_use] extern crate mime;
#[macro_use] extern crate lsio;
extern crate aws_sdk_rust;

extern crate tokio_http2;
extern crate tokio_proto;
extern crate tokio_service;
extern crate tokio_core;

use std::io;
use std::env;
use std::path::PathBuf;
use std::convert::AsRef;
use std::time::Duration;

use futures::future;
use tokio_http2::http::{Request, Response, Http};
use tokio_proto::TcpServer;
use tokio_service::Service;
use tokio_core::net::TcpStream;

use clap::Shell;
use url::Url;

use lsio::config::ConfigFile;

use aws_sdk_rust::aws::errors::s3::S3Error;
use aws_sdk_rust::aws::s3::endpoint::*;
use aws_sdk_rust::aws::s3::s3client::S3Client;
use aws_sdk_rust::aws::common::region::Region;
use aws_sdk_rust::aws::common::credentials::{AwsCredentialsProvider, DefaultCredentialsProviderSync};
use aws_sdk_rust::aws::common::request::DispatchSignedRequest;

use routes::routes;

mod cli;
//mod router;
mod config;
mod routes;
mod http;
mod files;
mod api;

// Used for outbound calls.
static DEFAULT_USER_AGENT: &'static str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

// Make generic so as to be easier to create new servers...
#[derive(Clone, Copy, Debug)]
struct ServiceCall;

impl Service for ServiceCall {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = future::Ok<Response, io::Error>; //http::Error>;
    // type Error = http::Error;
    // type Future = ::futures::Finished<Response, http::Error>;

    fn call(&mut self, req: Request) -> Self::Future {

        // println!("{:#?}", req);

        // static mut gc: u16 = 1;
        // unsafe {
        //     println!("{}", gc);
        //     gc += 1;
        // }

        future::ok(routes(req))
        // ::futures::finished(routes(req))
    }
}

fn main() {
    pretty_env_logger::init();

    let mut is_quiet: bool = false;
    let mut is_time: bool = false;
    let mut is_bench: bool = false;
    let mut is_bucket_virtual: bool = true;

    let app = env!("CARGO_PKG_NAME");
    let config_dir = "/etc/lsio";
    let version = format!("{}", crate_version!());

    let matches = cli::build_cli(app, config_dir, &version).get_matches();

    if matches.is_present("generate-bash-completions") {
        cli::build_cli(app, config_dir, &version)
            .gen_completions_to(app, Shell::Bash, &mut io::stdout());
        ::std::process::exit(0);
    }

    // If the -q or --quiet flag was passed then shut off all output
    if matches.is_present("quiet") {
        is_quiet = true;
    }

    // If the -t or --time flag was passed then track operation time
    if matches.is_present("time") {
        is_time = true;
    }

    // If the -h or --bucket-virtual-host flag was passed then track operation time
    if matches.is_present("bucket-virtual-host") {
        is_bucket_virtual = false;
    }

    // NOTE: Get parameters or config for region, signature etc
    // Safe to unwrap since a default value is passed in. If a panic occurs then the environment
    // does not support a home directory.

    let region = match matches.value_of("region").unwrap().to_string().to_lowercase().as_ref() {
        "uswest1" => Region::UsWest1,
        "uswest2" => Region::UsWest2,
        "cnnorth1" => Region::CnNorth1,
        "eucentral1" => Region::EuCentral1,
        "euwest1" => Region::EuWest1,
        "saeast1" => Region::SaEast1,
        "apnortheast1" => Region::ApNortheast1,
        "apnortheast2" => Region::ApNortheast2,
        "apsouth1" => Region::ApSouth1,
        "apsoutheast1" => Region::ApSoutheast1,
        "apsoutheast2" => Region::ApSoutheast2,
        _ => Region::UsEast1,
    };

    // Option so None will be return if nothing is passed in.
    let ep_str = matches.value_of("endpoint");
    let proxy_str = matches.value_of("proxy");
    let signature_str = matches.value_of("signature");
    let bench = matches.value_of("bench");
    let user_agent = matches.value_of("user-agent").unwrap_or(DEFAULT_USER_AGENT);
    let ip = matches.value_of("ip").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("8000");

    // Override some parameters when bench is specified...
    if bench.is_some() {
        is_quiet = true;
        is_time = true;
        is_bench = true;
    }

    // Set the config_file path to the default if a value is empty or set it to the passed in path value
    let config_dir = matches.value_of("config").unwrap();
    let mut config_file = PathBuf::new();

    if config_dir.is_empty() {
        config_file.push("/etc/lsio/config");
    } else {
        config_file.push(config_dir);
    }

    let mut config = config::Config::from_file(config_file).unwrap_or(config::Config::default());

    // Let CLI args override any config setting if they exists.
    if ep_str.is_some() {
        config.set_endpoint(Some(Url::parse(ep_str.unwrap()).unwrap()));
    }

    if proxy_str.is_some() {
        config.set_proxy(Some(Url::parse(proxy_str.unwrap()).unwrap()));
    }

    // if ip.is_some() {
    //     config.set_ip(Some(ip.unwrap()));
    // }

    if signature_str.is_some() {
        config.set_signature(signature_str.unwrap().to_string());
    } else {
        config.set_signature("V4".to_string());
    }

    // let sign: String = config.signature.to_lowercase();
    // let provider = DefaultCredentialsProviderSync::new(None).unwrap();
    // let endpoint = Endpoint::new(region,
    //                              if sign == "v2" {Signature::V2} else {Signature::V4},
    //                              config.clone().endpoint,
    //                              config.clone().proxy,
    //                              Some(user_agent.to_string()),
    //                              Some(is_bucket_virtual));
    //
    // let mut s3client = S3Client::new(provider, endpoint);

    let addr: &str = &format!("{}:{}", ip, port);
    let addr = addr.parse().unwrap();
    println!("{} - listening on http://{}", app, addr);

    let mut srv = TcpServer::new(Http, addr);
    srv.threads(num_cpus::get());

    srv.serve(|| Ok(ServiceCall));
}
