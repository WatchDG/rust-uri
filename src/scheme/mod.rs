use string_repr::StringRepr;

pub enum Scheme<'a> {
    HTTP,
    HTTPS,
    CUSTOM(&'a str),
}

impl<'a> Scheme<'a> {
    pub fn new(data: &str) -> Scheme {
        Scheme::CUSTOM(data)
    }
}

impl<'a> StringRepr for Scheme<'a> {
    fn string_repr(&self) -> String {
        match self {
            Scheme::HTTP => String::from("http"),
            Scheme::HTTPS => String::from("https"),
            Scheme::CUSTOM(string) => String::from(*string),
        }
    }
}

#[macro_export]
macro_rules! scheme {
    ($scheme:expr) => {
        Scheme::new($scheme)
    };
}
