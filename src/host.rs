pub struct Host(String);

impl Host {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn host(&self) -> String {
        String::from(&self.0)
    }
}

impl From<&str> for Host {
    fn from(data: &str) -> Self {
        Self(String::from(data))
    }
}

impl From<String> for Host {
    fn from(data: String) -> Self {
        Self(data)
    }
}
