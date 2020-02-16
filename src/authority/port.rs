use std::fmt;
use string_repr::StringRepr;

pub struct Port(String);

impl Port {
    pub fn new(data: String) -> Port {
        Port(data)
    }
}

impl StringRepr for Port {
    fn string_repr(&self) -> String {
        self.0.clone()
    }
}

impl fmt::Display for Port {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "port: {}", self.0)
    }
}
