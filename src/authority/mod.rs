mod host;
mod port;
mod user_info;

pub use host::*;
pub use port::*;
use string_repr::StringRepr;
pub use user_info::*;

pub struct Authority {
    host: Host,
    port: Option<Port>,
    user_info: Option<UserInfo>,
}

impl StringRepr for Authority {
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

impl Authority {
    pub fn new(host: Host) -> Authority {
        Authority {
            host,
            port: None,
            user_info: None,
        }
    }
    pub fn set_port(&mut self, port: Port) {
        self.port = Some(port);
    }
    pub fn set_user_info(&mut self, user_info: UserInfo) {
        self.user_info = Some(user_info);
    }
}
