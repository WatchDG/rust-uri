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

pub struct QueryKeyValueComponent(String, String);

impl QueryKeyValueComponent {
    pub fn new(key: String, value: String) -> Self {
        Self(key, value)
    }
    pub fn encode(&mut self) -> Result<(), Box<dyn Error>> {
        self.0 = pct_encode(&self.0)?;
        self.1 = pct_encode(&self.1)?;
        Ok(())
    }
    pub fn kvc(&self) -> String {
        format!("{}={}", self.0, self.1)
    }
}

pub struct Query {
    key_value_components: Option<Vec<QueryKeyValueComponent>>,
}

impl Query {
    pub fn new() -> Self {
        Query {
            key_value_components: None,
        }
    }
    pub fn add_kvc(&mut self, data: QueryKeyValueComponent) {
        match &mut self.key_value_components {
            Some(kvcs) => {
                kvcs.push(data);
            }
            None => {
                let mut vec = Vec::<QueryKeyValueComponent>::new();
                vec.push(data);
                self.key_value_components = Some(vec);
            }
        }
    }
    pub fn q(&mut self) -> String {
        let mut s = String::new();
        match &mut self.key_value_components {
            Some(kvcs) => {
                let d = kvcs
                    .iter()
                    .map(|kvc| kvc.kvc())
                    .collect::<Vec<String>>()
                    .join("&");
                s.push_str("?");
                s.push_str(d.as_str());
            }
            None => {}
        }
        s
    }
}
