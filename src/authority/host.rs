use std::fmt;
use string_repr::StringRepr;

pub struct Host(String);

impl Host {
    pub fn new(data: String) -> Host {
        Host(data)
    }
}

impl StringRepr for Host {
    fn string_repr(&self) -> String {
        self.0.clone()
    }
}

impl fmt::Display for Host {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "host: {}", self.0)
    }
}

#[macro_export]
macro_rules! host {
    ($host: expr;!) => {
        Host::new($host)
    };
    ($host:expr) => {
        Host::new($host.into())
    };
}
