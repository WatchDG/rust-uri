use crate::scheme::Scheme;
use regex::Regex;
use string_repr::StringRepr;

#[macro_export]
macro_rules! port {
    ($port:expr) => {
        Port::new($port)
    };
}

lazy_static! {
    static ref PORT_RE: Regex = Regex::new(r"^\d*$").unwrap();
}

pub struct Port<'a>(&'a str);

impl<'a> Port<'a> {
    /// Create new Port.
    /// # Example:
    /// ```
    /// use wdg_uri::authority::Port;
    /// let port = Port::new("80");
    /// ```
    pub fn new(data: &str) -> Port {
        Port(data)
    }
    /// Validate Port.
    /// # Example:
    /// ```
    /// use wdg_uri::authority::Port;
    /// let port = Port::new("80");
    /// if !port.validate() {
    ///     panic!("fail");
    /// }
    /// ```
    pub fn validate(&self) -> bool {
        PORT_RE.is_match(&self.0)
    }
    pub fn default_for_scheme(scheme: Scheme) -> Option<Port> {
        match scheme {
            Scheme::HTTP => Some(Port("80")),
            Scheme::HTTPS => Some(Port("443")),
            Scheme::CUSTOM(_) => None,
        }
    }
}

impl<'a> StringRepr for Port<'a> {
    fn string_repr(&self) -> String {
        String::from(self.0)
    }
}
