use std::error::Error;

pub fn pct_encode(data: &String) -> Result<String, Box<dyn Error>> {
    let mut string = String::with_capacity(data.len());
    for byte in data.as_bytes().iter() {
        match *byte as char {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => string.push(*byte as char),
            byte => string.push_str(format!("%{:02X}", byte as u8).as_str()),
        }
    }
    Ok(string)
}

pub struct KeyValueComponent(String, String);

impl KeyValueComponent {
    pub fn new(key: String, value: String) -> KeyValueComponent {
        KeyValueComponent(key, value)
    }
    pub fn encode(&mut self) -> Result<(), Box<dyn Error>> {
        self.0 = pct_encode(&self.0)?;
        self.1 = pct_encode(&self.1)?;
        Ok(())
    }
}

impl ToString for KeyValueComponent {
    fn to_string(&self) -> String {
        format!("{}={}", self.0, self.1)
    }
}
