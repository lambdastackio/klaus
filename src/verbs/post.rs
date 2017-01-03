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

use std::{io, slice, str, fmt};

use tokio_http2::http::{Request, Response, Http};
use tokio_http2::StatusCode;
use rustc_serialize::json::*;
use rustc_serialize::base64::*;

pub fn route(req: Request) -> Response {
    match req.path() {
        "/admin/settings" => {
            post(req)
        },
        _ => Response::new().with_status(StatusCode::MethodNotAllowed),
    }
}

fn post(req: Request) -> Response {
    let mut has_payload: bool = false;

    match req.content_type() {
        "application/json" => {
            match req.payload() {
                Some(payload) => {
                    let data = Json::from_str(str::from_utf8(payload).unwrap_or("{}"));
                    has_payload = true;
                    println!("{}", data.unwrap().pretty());
                },
                None => {},
            }
        },
        "application/base64" => {
            match req.payload() {
                Some(payload) => {
                    // Since the FromBase64 trait is in scope above you can apply it to payload()
                    let data = payload.from_base64();
                    has_payload = true;
                    println!("{:?}", data.unwrap());
                },
                None => {},
            }
        },
        _ => {
            match req.payload() {
                Some(payload) => {
                    has_payload = true;
                    println!("{}", str::from_utf8(payload).unwrap_or(""));
                },
                None => {},
            }
        },
    }

    if has_payload {
        Response::new()
            .with_header("Server", "lsioHTTPS")
            .with_header("Content-Length", "0")
            .with_status(StatusCode::Ok)
    } else {
        Response::new()
            .with_header("Server", "lsioHTTPS")
            .with_header("Content-Length", "0")
            .with_status(StatusCode::BadRequest)
    }
}
