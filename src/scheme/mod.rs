use string_repr::StringRepr;

pub struct Scheme(String);

impl Scheme {
    pub fn new(data: String) -> Scheme {
        Scheme(data)
    }
}

impl StringRepr for Scheme {
    fn string_repr(&self) -> String {
        self.0.clone()
    }
}
