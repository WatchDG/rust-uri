use string_repr::StringRepr;

#[macro_export]
macro_rules! host {
    ($host:expr) => {
        Host::new($host)
    };
}

pub struct Host<'a>(&'a str);

impl<'a> Host<'a> {
    /// Create new Host.
    /// # Example:
    /// ```
    /// use wdg_uri::authority::Host;
    /// let host = Host::new("localhost");
    /// ```
    pub fn new(data: &str) -> Host {
        Host(data)
    }

    /// Validate Host.
    /// # Support:
    /// * IPv4Address
    /// * IPv6Address
    /// # Example:
    /// ```
    /// use wdg_uri::authority::Host;
    /// let host = Host::new("127.0.0.1");
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
    /// let host = Host::new("127.0.0.1");
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
    /// let host = Host::new("::");
    /// if host.is_ipv6addr() {
    ///    println!("Host is IPv6Address.");
    /// }
    /// ```
    pub fn is_ipv6addr(&self) -> bool {
        regexp::IP_V6_ADDR(&self.0)
    }
}

impl<'a> StringRepr for Host<'a> {
    fn string_repr(&self) -> String {
        String::from(self.0)
    }
}
