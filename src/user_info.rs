use std::fmt;

pub struct UserInfo(String);

impl fmt::Display for UserInfo {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "UserInfo({})", self.0)
    }
}

impl UserInfo {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn user_info(&self) -> String {
        String::from(&self.0)
    }
}
