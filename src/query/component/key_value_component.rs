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

pub enum KeyValueBuilderConfig {
    CUSTOM(KeyValueBuilderConfigEncode),
}

pub enum KeyValueBuilderConfigEncode {
    KEY,
    VALUE,
    BOTH,
    NONE,
}

pub struct KeyValue<'a>(&'a str, &'a str);

impl<'a> KeyValue<'a> {
    pub fn new(key: &'a str, value: &'a str) -> KeyValue<'a> {
        KeyValue(key, value)
    }
    pub fn build(
        key: &'a str,
        value: &'a str,
        config: Option<KeyValueBuilderConfig>,
    ) -> Result<KeyValue<'a>, Box<dyn Error>> {
        match config {
            Some(cfg) => match cfg {
                KeyValueBuilderConfig::CUSTOM(encode_config) => match encode_config {
                    KeyValueBuilderConfigEncode::KEY => {
                        Ok(KeyValue::new(Box::leak(Box::new(pct_encode(key)?)), value))
                    }
                    KeyValueBuilderConfigEncode::VALUE => {
                        Ok(KeyValue::new(key, Box::leak(Box::new(pct_encode(value)?))))
                    }
                    KeyValueBuilderConfigEncode::BOTH => Ok(KeyValue::new(
                        Box::leak(Box::new(pct_encode(key)?)),
                        Box::leak(Box::new(pct_encode(value)?)),
                    )),
                    KeyValueBuilderConfigEncode::NONE => Ok(KeyValue::new(key, value)),
                },
            },
            None => Ok(KeyValue::new(
                Box::leak(Box::new(pct_encode(key)?)),
                Box::leak(Box::new(pct_encode(value)?)),
            )),
        }
    }
}

impl<'a> StringRepr for KeyValue<'a> {
    fn string_repr(&self) -> String {
        format!("{}={}", self.0, self.1)
    }
}
