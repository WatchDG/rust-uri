extern crate regex;

use regex::Regex;
use std::fmt;

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

impl From<&str> for Port {
    fn from(data: &str) -> Self {
        Self(String::from(data))
    }
}

impl From<String> for Port {
    fn from(data: String) -> Self {
        Self(data)
    }
}
