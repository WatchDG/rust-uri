pub struct Host(String);

impl Default for Host {
    fn default() -> Host {
        Host(String::from("localhost"))
    }
}

impl ToString for Host {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl Host {
    pub fn new(data: String) -> Host {
        Host(data)
    }
}
