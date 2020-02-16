use string_repr::StringRepr;

pub struct UserInfo(String);

impl StringRepr for UserInfo {
    fn string_repr(&self) -> String {
        self.0.clone()
    }
}

impl UserInfo {
    pub fn new(data: String) -> UserInfo {
        UserInfo(data)
    }
}
