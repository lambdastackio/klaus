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

#![allow(dead_code)]  // Implement this for when you don't want warning for any of the functions below that you're not using.

use tokio_http2::{Router, Route, RouterBuilder, Logger, LoggerLevel};
use tokio_http2::http::{Request, Response};
use routes::routes;

/// Handlers - Top level function to build routes
pub fn handlers() -> Option<RouterBuilder> {
    None
    // Some(RouterBuilder::new().add(Route::get(r"/index2.html").using(test_handler)))
}

fn test_handler(req: Request) -> Response {
    if req.logger.is_some() {
        let logger = req.logger.clone().unwrap();
        logger.write(LoggerLevel::Warn, format!("Handler - line: {}, col: {}, mod: {}", line!(), column!(), module_path!()));
    }
    routes(req)
}
