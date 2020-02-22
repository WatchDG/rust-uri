use string_repr::StringRepr;

pub struct Path(String);

impl Path {
    /// Create new Path.
    /// # Example:
    /// ```
    /// use wdg_uri::path::Path;
    /// let path = Path::new("/".into());
    /// ```
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
