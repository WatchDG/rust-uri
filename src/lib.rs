pub mod host;
pub mod port;
pub mod user_info;

use host::Host;
use port::Port;
use std::io::Error;
use user_info::UserInfo;

pub struct Authority {
    host: Host,
    port: Option<Port>,
    user_info: Option<UserInfo>,
}

impl Default for Authority {
    fn default() -> Self {
        Self {
            host: Host::new("localhost".into()),
            port: None,
            user_info: None,
        }
    }
}

impl Authority {
    pub fn new(host: Host) -> Self {
        Authority {
            host,
            ..Self::default()
        }
    }
    pub fn build(host: Host, port: Option<Port>, user_info: Option<UserInfo>) -> Self {
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
        string.push_str(self.host.host().as_str());
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
