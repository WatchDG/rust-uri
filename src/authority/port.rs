use std::fmt;
use string_repr::StringRepr;

pub struct Port(String);

impl Port {
    /// Create new Port.
    /// # Example:
    /// ```
    /// use wdg_uri::authority::Port;
    /// let port = Port::new("80".into());
    /// ```
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

#[macro_export]
macro_rules! port {
    ($port: expr;!) => {
        Port::new($port)
    };
    ($port:expr) => {
        Port::new($port.into())
    };
}
