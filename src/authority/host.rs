use string_repr::StringRepr;

pub struct Host(String);

impl StringRepr for Host {
    fn string_repr(&self) -> String {
        self.0.clone()
    }
}

impl Host {
    pub fn new(data: String) -> Host {
        Host(data)
    }
}
