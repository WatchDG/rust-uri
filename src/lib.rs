use std::io::Error;

pub struct UserInfo {
    data: String,
}

impl UserInfo {
    pub fn new(data: String) -> UserInfo {
        UserInfo { data }
    }
    pub fn user_info(&self) -> String {
        String::from(&self.data)
    }
}

pub struct Authority {
    host: String,
    port: Option<String>,
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
    pub fn build(host: String, port: Option<String>, user_info: Option<UserInfo>) -> Self {
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
            Some(port) => string.push_str(format!(":{}", port).as_str()),
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
