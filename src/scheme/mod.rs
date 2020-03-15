use default_port::DefaultPort;
use string_repr::StringRepr;

#[macro_export]
macro_rules! scheme {
    ($scheme:expr) => {
        Scheme::new($scheme)
    };
}

pub enum Scheme<'a> {
    HTTP,
    HTTPS,
    CUSTOM(&'a str),
}

impl<'a> Scheme<'a> {
    /// Create new Scheme.
    /// # Example:
    /// ```rust
    /// use wdg_uri::scheme::Scheme;
    /// let custom_scheme = Scheme::new("http");
    /// ```
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

impl<'a> DefaultPort for Scheme<'a> {
    fn default_port(&self) -> Option<usize> {
        match self {
            Scheme::HTTP => Some(80),
            Scheme::HTTPS => Some(443),
            _ => None,
        }
    }
}
