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

pub fn read(path: &str) -> Result<FileBody, io::Error> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            return Err(e);
        }
    };

    let mut content_length: ContentLength = 0;
    let content_type: &str;
    let ext: String;

    match Path::new(path).extension() {
        Some(s) => {
            ext = s.to_str().unwrap_or("").to_string();
            match s.to_str() {
                Some("bmp") => content_type = "image/bmp", //ContentType(mime!(Image/Bmp)),
                Some("css") => content_type = "text/css", //ContentType(mime!(Text/Css)), //; Charset=Utf8)),
                Some("gif") => content_type = "image/gif", //ContentType(mime!(Image/Gif)),
                Some("html") => content_type = "text/html", //ContentType(mime!(Text/Html; Charset=Utf8)),
                Some("ico") => content_type = "image/x-icon", //ContentType(Mime::from_str("image/x-icon").unwrap()),
                Some("js") => content_type = "application/javascript", //ContentType(mime!(Application/Javascript)),
                Some("json") => content_type = "application/json", //ContentType(mime!(Application/Json)),
                Some("jpg") | Some("jpeg") => content_type = "image/jpeg", //ContentType(mime!(Image/Jpeg)),
                Some("mp4") | Some("mpeg") => content_type = "video/mp4", //ContentType(mime!(Video/Mp4)),
                Some("png") => content_type = "image/png", //ContentType(mime!(Image/Png)),
                Some("txt") => content_type = "text/plain", //ContentType(mime!(Text/Plain; Charset=Utf8)),
                Some("xml") => content_type = "text/xml", //ContentType(mime!(Text/Xml; Charset=Utf8)),
                None | Some(&_) => content_type = "application/octetstream", //ContentType(mime!(Application/OctetStream)),
            }
        },
        None => {
            ext = "".to_string();
            content_type = "application/octetstream"; //ContentType(mime!(Application/OctetStream));
        },
    }

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
