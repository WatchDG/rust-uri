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
