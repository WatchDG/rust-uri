use std::error::Error;
use string_repr::StringRepr;

pub fn pct_encode(data: &str) -> Result<String, Box<dyn Error>> {
    let mut string = String::with_capacity(data.len());
    for byte in data.as_bytes().iter() {
        match *byte as char {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => string.push(*byte as char),
            byte => string.push_str(format!("%{:02X}", byte as u8).as_str()),
        }
    }
    Ok(string)
}

pub struct KeyValueComponent<'a>(&'a str, &'a str);

impl<'a> KeyValueComponent<'a> {
    pub fn new(key: &'a str, value: &'a str) -> KeyValueComponent<'a> {
        KeyValueComponent(key, value)
    }
    //    pub fn encode(&mut self) -> Result<(), Box<dyn Error>> {
    //        self.0 = pct_encode(self.0)?.as_str();
    //        self.1 = pct_encode(self.1)?.as_str();
    //        Ok(())
    //    }
}

impl<'a> StringRepr for KeyValueComponent<'a> {
    fn string_repr(&self) -> String {
        format!("{}={}", self.0, self.1)
    }
}
