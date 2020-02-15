mod host;
mod port;
mod user_info;

pub use host::*;
pub use port::*;
pub use user_info::*;

pub struct Authority {
    host: Host,
    port: Option<Port>,
    user_info: Option<UserInfo>,
}

impl Default for Authority {
    fn default() -> Self {
        Self {
            host: Host::new("localhost".into()),
            port: None,
            user_info: None,
        }
    }
}

impl Authority {
    pub fn new(host: Host) -> Self {
        Authority {
            host,
            ..Self::default()
        }
    }
    pub fn build(host: Host, port: Option<Port>, user_info: Option<UserInfo>) -> Self {
        Self {
            host,
            port,
            user_info,
        }
    }
    pub fn authority(&self) -> String {
        let mut string = String::new();
        match &self.user_info {
            Some(user_info) => string.push_str(format!("{}@", user_info.to_string()).as_str()),
            None => {}
        }
        string.push_str(self.host.to_string().as_str());
        match &self.port {
            Some(port) => string.push_str(format!(":{}", port.to_string()).as_str()),
            None => {}
        }
        string
    }
}
