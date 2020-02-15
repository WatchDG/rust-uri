extern crate regex;

use regex::Regex;

pub struct Port(String);

impl ToString for Port {
    fn to_string(&self) -> String {
        self.0.clone()
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
