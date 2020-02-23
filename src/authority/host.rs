use std::fmt;
use string_repr::StringRepr;

pub struct Host(String);

impl Host {
    /// Create new Host.
    /// # Example:
    /// ```
    /// use wdg_uri::authority::Host;
    /// let host = Host::new("localhost".into());
    /// ```
    pub fn new(data: String) -> Host {
        Host(data)
    }

    /// Validate Host.
    /// # Support:
    /// * IPv4Address
    /// * IPv6Address
    /// # Example:
    /// ```
    /// use wdg_uri::authority::Host;
    /// let host = Host::new("127.0.0.1".into());
    /// if !host.validate() {
    ///     panic!("fail");
    /// }
    /// ```
    pub fn validate(&self) -> bool {
        self.is_ipv4addr() | self.is_ipv6addr()
    }

    /// Check if Host is IPv4Address.
    /// # Example:
    /// ```
    /// use wdg_uri::authority::Host;
    /// let host = Host::new("127.0.0.1".into());
    /// if host.is_ipv4addr() {
    ///    println!("Host is IPv4Address.");
    /// }
    /// ```
    pub fn is_ipv4addr(&self) -> bool {
        regexp::IP_V4_ADDR(&self.0)
    }

    /// Check if Host is IPv6Address.
    /// # Example:
    /// ```
    /// use wdg_uri::authority::Host;
    /// let host = Host::new("127.0.0.1".into());
    /// if host.is_ipv4addr() {
    ///    println!("Host is IPv4Address.");
    /// }
    /// ```
    pub fn is_ipv6addr(&self) -> bool {
        regexp::IP_V6_ADDR(&self.0)
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
