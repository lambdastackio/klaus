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

use std::io::{Error, ErrorKind};

use tokio_http2::http::{Request, Response, Http};
use tokio_http2::StatusCode;

use files::*;

pub fn route(req: Request, prefix: String) -> Response {
    match req.path() {
        "" => {
            Response::new()
                .with_header("Server", "lsioHTTPS")
                .with_header("Content-Length", "0")
                .with_status(StatusCode::NoContent)
        },
        "/" | "/index.html" => {
            load_default(prefix)
        },
        file_path => {
            match read(&format!("{}{}", prefix, file_path)) {
                Ok(file_body) => {
                    Response::new()
                        .with_header("Server", "lsioHTTPS")
                        .with_header("Content-Length", &file_body.content_length.to_string())
                        .with_header("Content-Type", &file_body.content_type)
                        .with_body(file_body.body)
                        .with_status(StatusCode::Ok)
                },
                Err(e) => {
                    // NOTE: This defaults to the index page so that single page apps like React can
                    // determine a 404 error or show page source.

                    if e.kind() == ErrorKind::NotFound {
                        load_default(prefix)
                     } else {
                        Response::new()
                            .with_header("Server", "lsioHTTPS")
                            .with_header("Content-Length", "0")
                            .with_status(StatusCode::NotImplemented)
                    }
                }
            }
        },
    }
}

fn load_default(prefix: String) -> Response {
    // NOTE: Maybe add a list of acceptable index files in the config and check them in order instead of the static `index.html`
    match read(&format!("{}{}", prefix, "/index.html")) {
        Ok(file_body) => {
            Response::new()
                .with_header("Server", "lsioHTTPS")
                .with_header("Content-Length", &file_body.content_length.to_string())
                .with_header("Content-Type", &file_body.content_type)
                .with_body(file_body.body)
                .with_status(StatusCode::Ok)
        },
        Err(e) => {
            Response::new()
                .with_header("Server", "lsioHTTPS")
                .with_header("Content-Length", "0")
                .with_status(StatusCode::NotFound)
        }
    }
}
