pub mod component;

use component::KeyValueComponent;
use string_repr::StringRepr;

pub struct Query {
    key_value_components: Option<Vec<KeyValueComponent>>,
}

impl Default for Query {
    fn default() -> Query {
        Query {
            key_value_components: None,
        }
    }
}

impl StringRepr for Query {
    fn string_repr(&self) -> String {
        let mut s = String::new();
        match &self.key_value_components {
            Some(kvcs) => {
                let d = kvcs
                    .iter()
                    .map(|kvc| kvc.string_repr())
                    .collect::<Vec<String>>()
                    .join("&");
                s.push_str(d.as_ref());
            }
            None => {}
        }
        s
    }
}

impl Query {
    pub fn new() -> Query {
        Self::default()
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
