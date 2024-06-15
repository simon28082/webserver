use std::collections::HashMap;
use super::enums;

#[derive(Debug)]
pub struct Request {
    pub target: String,
    pub version: enums::Version,
    pub method: enums::Method,
    pub headers: Option<HashMap<String, Vec<String>>>,
    pub body: Option<Vec<u8>>,
}

// impl Request {
//     pub fn new(url: String, method: String, headers: HashMap<String,Vec<String>>) -> Self {
//         Request {
//             url,
//             method,
//             headers
//         }
//     }
// }