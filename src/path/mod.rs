use string_repr::StringRepr;

pub struct Path<'a>(&'a str);

impl<'a> Path<'a> {
    /// Create new Path.
    /// # Example:
    /// ```
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

#[macro_export]
macro_rules! path {
    ($path:expr) => {
        Path::new($path)
    };
}
