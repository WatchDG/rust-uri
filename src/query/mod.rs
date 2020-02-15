mod component;

pub use component::*;

pub struct Query {
    key_value_components: Option<Vec<KeyValueComponent>>,
}

impl Query {
    pub fn new() -> Query {
        Query {
            key_value_components: None,
        }
    }
    pub fn add_kvc(&mut self, data: KeyValueComponent) {
        match &mut self.key_value_components {
            Some(kvcs) => {
                kvcs.push(data);
            }
            None => {
                let mut vec = Vec::<KeyValueComponent>::new();
                vec.push(data);
                self.key_value_components = Some(vec);
            }
        }
    }
}

impl ToString for Query {
    fn to_string(&self) -> String {
        let mut s = String::new();
        match &self.key_value_components {
            Some(kvcs) => {
                let d = kvcs
                    .iter()
                    .map(|kvc| kvc.to_string())
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
