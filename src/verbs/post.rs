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

use tokio_http2::http::{Request, Response};
use tokio_http2::Method as Method;
use tokio_http2::StatusCode;

pub fn route(req: Request) -> Response {
    Response::new()

    // match req.path().unwrap_or("") {
    //     "/echo" => {
    //         let mut res = Response::new();
    //         if let Some(len) = req.headers().get::<ContentLength>() {
    //             res.headers_mut().set(len.clone());
    //         }
    //
    //         res.with_body(req.body())
    //     },
    //     _ => {
    //         Response::new()
    //             .with_status(StatusCode::NotFound)
    //     }
    // }
}
