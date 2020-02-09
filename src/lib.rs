use std::io::Error;

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

pub struct Authority {
    host: String,
    port: Option<String>,
}
impl Authority {
    pub fn new(host: String, port: Option<String>) -> Authority {
        Authority { host, port }
    }
    pub fn authority(&self) -> String {
        let mut string = String::new();
        string.push_str(self.host.as_str());
        match &self.port {
            Some(port) => string.push_str(format!(":{}", port).as_str()),
            None => {}
        }
        string
    }
}
