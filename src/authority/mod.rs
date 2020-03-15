mod host;
mod port;
mod user_info;

pub use host::*;
pub use port::*;
use regex::{Error, Regex};
use string_repr::StringRepr;
pub use user_info::*;

#[macro_export]
macro_rules! authority {
    ($host: expr) => {
        Authority::new($host)
    };
    ($host:expr;$port:expr) => {{
        let mut authority = Authority::new($host);
        authority.set_port($port);
        authority
    }};
    ($user_info:expr;$host:expr;$port:expr) => {{
        let mut authority = Authority::new($host);
        authority.set_port($port);
        authority.set_user_info($user_info);
        authority
    }};
    ($user_info:expr;$host:expr;) => {{
        let mut authority = Authority::new($host);
        authority.set_user_info($user_info);
        authority
    }};
}

pub struct Authority<'a> {
    host: Host<'a>,
    port: Option<Port<'a>>,
    user_info: Option<UserInfo<'a>>,
}

impl<'a> Authority<'a> {
    pub fn new(host: Host<'a>) -> Authority<'a> {
        Authority {
            host,
            port: None,
            user_info: None,
        }
    }
    pub fn get_host(&self) -> &Host {
        &self.host
    }
    pub fn get_port(&self) -> Option<&Port> {
        match &self.port {
            Some(port) => Some(port),
            None => None,
        }
    }
    pub fn set_port(&mut self, port: Port<'a>) {
        self.port = Some(port);
    }
    pub fn get_user_info(&self) -> Option<&UserInfo> {
        match &self.user_info {
            Some(user_info) => Some(user_info),
            None => None,
        }
    }
    pub fn set_user_info(&mut self, user_info: UserInfo<'a>) {
        self.user_info = Some(user_info);
    }
}

impl<'a> StringRepr for Authority<'a> {
    fn string_repr(&self) -> String {
        let mut string = String::new();
        match &self.user_info {
            Some(user_info) => string.push_str(format!("{}@", user_info.string_repr()).as_str()),
            None => {}
        }
        string.push_str(self.host.string_repr().as_str());
        match &self.port {
            Some(port) => string.push_str(format!(":{}", port.string_repr()).as_str()),
            None => {}
        }
        string
    }
}

pub trait AuthorityParse {
    fn parse(&self) -> Result<Authority<'_>, Error>;
}

impl AuthorityParse for &str {
    fn parse(&self) -> Result<Authority, Error> {
        lazy_static! {
            static ref AUTHORITY_PARSE_RE: Regex = Regex::new(
                r"(?x)
            ^(?:([^@])+@)?(?P<host>[^:]+)(?::(?P<port>.*))?$
            "
            )
            .unwrap();
        }
        let captures = regexp::uri::authority::parse::RE.captures(&self).unwrap();
        Ok(Authority {
            host: captures
                .name("host")
                .map(|host| Host::new(host.as_str()))
                .unwrap(),
            port: captures
                .name("port")
                .map_or(None, |port| Some(Port::new(port.as_str()))),
            user_info: captures
                .name("user_info")
                .map_or(None, |user_info| Some(UserInfo::new(user_info.as_str()))),
        })
    }
}
