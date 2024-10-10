use core::fmt;
use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
};

pub enum RequestType {
    GET,
    SET,
    REMOVE,
}

// ---RequestType---
impl fmt::Display for RequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str;
        match self {
            RequestType::GET => str = "GET",
            RequestType::SET => str = "SET",
            RequestType::REMOVE => str = "REMOVE",
        }

        write!(f, "{}", str)
    }
}

pub struct Request {
    pub reqtype: RequestType,
    pub key: String,
    pub val: Option<String>,
}

// ---Request---
impl Request {
    pub fn parse(stream: TcpStream) -> Option<Self> {
        let mut reader =
            BufReader::new(stream.try_clone().expect("Request: Failed to clone stream"));

        let mut command_line = String::new();
        if reader.read_line(&mut command_line).is_err() {
            return None;
        }

        let command = command_line.trim();
        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts.as_slice() {
            ["GET", key] => {
                return Some(Request {
                    reqtype: RequestType::GET,
                    key: key.to_string(),
                    val: None,
                })
            }
            ["SET", key, val] => {
                return Some(Request {
                    reqtype: RequestType::SET,
                    key: key.to_string(),
                    val: Some(val.to_string()),
                })
            }
            ["REMOVE", key] => {
                return Some(Request {
                    reqtype: RequestType::REMOVE,
                    key: key.to_string(),
                    val: None,
                })
            }
            _ => return None,
        }
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: {} {}",
            self.reqtype,
            self.key,
            self.val.as_ref().unwrap_or(&"".to_string())
        )
    }
}
