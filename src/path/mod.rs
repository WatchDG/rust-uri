use string_repr::StringRepr;

#[macro_export]
macro_rules! path {
    ($path:expr) => {
        Path::new($path)
    };
}

pub struct Path<'a>(&'a str);

impl<'a> Path<'a> {
    /// Create new Path.
    /// # Example:
    /// ```rust
    /// use wdg_uri::path::Path;
    /// let path = Path::new("/");
    /// ```
    pub fn new(data: &str) -> Path {
        Path(data)
    }
}

impl<'a> StringRepr for Path<'a> {
    fn string_repr(&self) -> String {
        String::from(self.0)
    }
}
