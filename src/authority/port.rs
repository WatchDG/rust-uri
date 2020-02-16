use string_repr::StringRepr;

pub struct Port(String);

impl StringRepr for Port {
    fn string_repr(&self) -> String {
        self.0.clone()
    }
}

impl Port {
    pub fn new(data: String) -> Port {
        Port(data)
    }
}
