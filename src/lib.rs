extern crate regex;

use regex::Regex;
use std::fmt;
use std::io::Error;

pub struct Port(String);

impl fmt::Display for Port {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "Port({})", self.0)
    }
}

impl Port {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn validate(&self) -> bool {
        Regex::new(r"^\d*$").unwrap().is_match(self.0.as_str())
    }
    pub fn port(&self) -> String {
        String::from(&self.0)
    }
}

pub struct UserInfo(String);

impl fmt::Display for UserInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "UserInfo({})", self.0)
    }
}

impl UserInfo {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn user_info(&self) -> String {
        String::from(&self.0)
    }
}

pub struct Authority {
    host: String,
    port: Option<Port>,
    user_info: Option<UserInfo>,
}

impl Default for Authority {
    fn default() -> Self {
        Self {
            host: "localhost".into(),
            port: None,
            user_info: None,
        }
    }
}

impl Authority {
    pub fn new(host: String) -> Self {
        Authority {
            host,
            ..Self::default()
        }
    }
    pub fn build(host: String, port: Option<Port>, user_info: Option<UserInfo>) -> Self {
        Self {
            host,
            port,
            user_info,
        }
    }
    pub fn authority(&self) -> String {
        let mut string = String::new();
        match &self.user_info {
            Some(user_info) => string.push_str(format!("{}@", user_info.user_info()).as_str()),
            None => {}
        }
        string.push_str(self.host.as_str());
        match &self.port {
            Some(port) => string.push_str(format!(":{}", port.port()).as_str()),
            None => {}
        }
        string
    }
}

pub struct Query {
    pub data: String,
}

impl Query {
    pub fn new(data: String) -> Query {
        Query { data }
    }
    pub fn encode(data: String) -> Result<Query, Error> {
        let mut string = String::with_capacity(data.len());
        for byte in data.as_bytes().iter() {
            match *byte as char {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                    string.push(*byte as char)
                }
                byte => string.push_str(format!("%{:02X}", byte as u8).as_str()),
            }
        }
        Ok(Query { data: string })
    }
}
