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

#![allow(unused_assignments)]

use std::io;
use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

use tokio_http2::{Body, ContentType, ContentLength};
use mime_guess::guess_mime_type;
use mime::*;

#[derive(Debug)]
pub struct FileBody {
    pub content_length: ContentLength,
    pub content_type: ContentType,
    pub ext: String,
    pub body: Body,
}

impl FileBody {
    pub fn new(content_length: ContentLength, content_type: ContentType, ext: String, body: Body) -> FileBody {
        FileBody{ content_length: content_length, content_type: content_type, ext: ext, body: body }
    }
}

// TODO: Pass in Logger

pub fn read(path: &str) -> Result<FileBody, io::Error> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    };

    let mut content_length: ContentLength = 0;

    let path2 = Path::new(path);
    let ext: String;

    println!("read - {:?}", path2);

    match path2.extension() {
        Some(s) => ext = s.to_str().unwrap_or("").to_string(),
        None => ext = "".to_string(),
    }
    let mime = guess_mime_type(path2);
    let content_type_string = mime.to_string();
    let content_type: &str = &content_type_string;

    let mut body = Body::new();

    match file.read_to_end(&mut body) {
        Ok(len) => {
            content_length = len as u64;
        },
        Err(e) => {
            return Err(e);
        }
    }

    Ok(FileBody::new(content_length, content_type.to_string(), ext, body))
}
