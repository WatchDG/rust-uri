use string_repr::StringRepr;

pub struct Path(String);

impl Path {
    pub fn new(data: String) -> Path {
        Path(data)
    }
}

impl StringRepr for Path {
    fn string_repr(&self) -> String {
        self.0.clone()
    }
}

#[macro_export]
macro_rules! path {
    ($path: expr;!) => {
        Path::new($path)
    };
    ($path:expr) => {
        Path::new($path.into())
    };
}
