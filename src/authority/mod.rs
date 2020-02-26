mod host;
mod port;
mod user_info;

pub use host::*;
pub use port::*;
use string_repr::StringRepr;
pub use user_info::*;

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
    pub fn set_port(&mut self, port: Port<'a>) {
        self.port = Some(port);
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
