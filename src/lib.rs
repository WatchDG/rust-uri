#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate regexp;
extern crate string_repr;

pub mod authority;
pub mod path;
pub mod query;
pub mod scheme;

use authority::Authority;
use path::Path;
use query::Query;
use scheme::Scheme;
use string_repr::StringRepr;

pub struct URI<'a> {
    pub scheme: Scheme<'a>,
    pub authority: Option<Authority<'a>>,
    pub path: Path<'a>,
    pub query: Option<Query<'a>>,
}

impl<'a> URI<'a> {
    pub fn new(scheme: Scheme<'a>, path: Path<'a>) -> URI<'a> {
        URI {
            scheme,
            authority: None,
            path,
            query: None,
        }
    }
    pub fn set_authority(&mut self, authority: Authority<'a>) {
        self.authority = Some(authority);
    }
    pub fn set_query(&mut self, query: Query<'a>) {
        self.query = Some(query);
    }
}

impl<'a> StringRepr for URI<'a> {
    fn string_repr(&self) -> String {
        let mut string = format!("{}:", self.scheme.string_repr());
        match &self.authority {
            Some(authority) => string.push_str(format!("//{}", authority.string_repr()).as_ref()),
            None => {}
        }
        string.push_str(self.path.string_repr().as_ref());
        match &self.query {
            Some(query) => string.push_str(format!("?{}", query.string_repr()).as_ref()),
            None => {}
        }
        string
    }
}

#[macro_export]
macro_rules! uri {
    ($scheme:expr;$path:expr) => {
        URI::new($scheme, $path)
    };
    ($scheme:expr;$authority:expr;$path:expr) => {{
        let mut uri = URI::new($scheme, $path);
        uri.set_authority($authority);
        uri
    }};
    ($scheme:expr;$authority:expr;$path:expr;$query:expr) => {{
        let mut uri = URI::new($scheme, $path);
        uri.set_authority($authority);
        uri.set_query($query);
        uri
    }};
}
