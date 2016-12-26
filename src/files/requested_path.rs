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

use std::iter::FromIterator;
use std::path::{Component, PathBuf, Path};
use std::fs::{self, Metadata};
use std::convert::AsRef;

use url::percent_encoding::percent_decode;

use hyper::server::Request;

pub struct RequestedPath {
    pub path: PathBuf,
}

#[inline]
fn decode_percents(string: &&str) -> String {
    percent_decode(string.as_bytes()).decode_utf8().unwrap().into_owned()
}

fn normalize_path(path: &Path) -> PathBuf {
    path.components().fold(PathBuf::new(), |mut result, p| {
        match p {
            Component::Normal(x) => {
                result.push(x);
                result
            }
            Component::ParentDir => {
                result.pop();
                result
            },
            _ => result
        }
    })
}

impl RequestedPath {
    pub fn new<P: AsRef<Path>>(root_path: P, request: &Request) -> RequestedPath {
        let decoded_req_path = PathBuf::from_iter(request.uri().path().iter().map(decode_percents));
        let mut result = root_path.as_ref().to_path_buf();
        result.extend(&normalize_path(&decoded_req_path));
        RequestedPath { path: result }
    }

    pub fn should_redirect(&self, metadata: &Metadata, request: &Request) -> bool {
        // As per servo/rust-url/serialize_path, URLs ending in a slash have an
        // empty string stored as the last component of their path. Rust-url
        // even ensures that url.path() is non-empty by appending a forward slash
        // to URLs like http://example.com
        // Some middleware may mutate the URL's path to violate this property,
        // so the empty list case is handled as a redirect.
        let has_trailing_slash = match request.uri().path().last() {
            Some(&"") => true,
            _ => false,
        };

        metadata.is_dir() && !has_trailing_slash
    }

    pub fn get_file(self, metadata: &Metadata) -> Option<PathBuf> {
        if metadata.is_file() {
            return Some(self.path);
        }

        let index_path = self.path.join("index.html");

        match fs::metadata(&index_path) {
            Ok(m) =>
                if m.is_file() {
                    Some(index_path)
                } else {
                    None
                },
            Err(_) => None,
        }
    }
}
