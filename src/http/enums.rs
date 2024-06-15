use std::io;
use crate::http::enums::Version::{V1_1, V2_0};
use crate::errors::parse::ParseError;
#[derive(Debug)]
pub enum Method {
    Post,
    Get,
    Delete,
    Put,
    Patch
}
// type Err = ();
pub fn parse_method_from_str(input : &str) -> Result<Method, ParseError>{
    match input.to_lowercase().as_str() {
        "post" => Ok(Method::Post),
        "get" => Ok(Method::Get),
        "delete" => Ok(Method::Delete),
        "put" => Ok(Method::Put),
        "patch" => Ok(Method::Patch),
        _ => Err(ParseError::from_str("no available http method"))
    }
}

#[derive(Debug)]
pub enum Version {
    V1_1,
    V2_0
}

pub fn parse_version_from_str(input: &str) -> Result<Version, ParseError> {
    match input.to_lowercase().as_str() {
        "http 1.1" => Ok(V1_1),
        "http 2.0" => Ok(V2_0),
        _ => Err(ParseError::from_str("no available http version"))
    }
}