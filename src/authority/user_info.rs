pub struct UserInfo(String);

impl ToString for UserInfo {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl UserInfo {
    pub fn new(data: String) -> UserInfo {
        UserInfo(data)
    }
}
