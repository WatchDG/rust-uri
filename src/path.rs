pub struct Path(String);

impl Path {
    pub fn new(data: String) -> Self {
        Self(data)
    }
    pub fn path(&self) -> String {
        String::from(&self.0)
    }
}
