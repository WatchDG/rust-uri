use string_repr::StringRepr;

pub struct UserInfo<'a>(&'a str);

impl<'a> StringRepr for UserInfo<'a> {
    fn string_repr(&self) -> String {
        String::from(self.0)
    }
}

impl<'a> UserInfo<'a> {
    pub fn new(data: &str) -> UserInfo {
        UserInfo(data)
    }
}
