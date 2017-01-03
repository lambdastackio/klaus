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

// use hyper::Method as Method;
// use hyper::server::{Request, Response};
// use hyper::{Get, Post, StatusCode};

use tokio_http2::http::{Request, Response, Http};
// use tokio_http2::server::{Request, Response};
// use tokio_http2::Method as Method;
use tokio_http2::StatusCode;

use verbs::*;

pub fn routes(req: Request) -> Response {
    match req.method() {
        "GET" => {
        // &Method::Get => {
            get::route(req, "public".to_string())
        },
        "POST" => {
        // &Method::Post => {
            post::route(req)
        },
        "PUT" => {
        // &Method::Put => {
            put::route(req)
        },
        "DELETE" => {
        // &Method::Delete => {
            delete::route(req)
        },
        "HEAD" => {
        // &Method::Head => {
            head::route(req)
        },
        _ => {
            Response::new()
                .with_status(StatusCode::NotFound)
        }
    }
}
