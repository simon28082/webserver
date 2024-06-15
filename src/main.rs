mod http;
mod errors;

use std::collections::HashMap;
use http::request::Request;
use crate::http::enums::{Method, Version};

fn main() {
    let headers: HashMap<String,String> = HashMap::new();
    // let req = Request::new(String::from("https://google.com"),String::from("POST"), headers);
    let req = Request{
        target: String::from("https://google.com.hk"),
        version: Version::V1_1,
        method: Method::Get,
        headers: None,
        body: None,
    };
    println!("{:#?}", req)
}
