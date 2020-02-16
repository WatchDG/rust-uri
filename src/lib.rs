extern crate string_repr;

pub mod authority;
pub mod path;
pub mod query;
pub mod scheme;

use authority::Authority;
use path::Path;
use scheme::Scheme;
use string_repr::StringRepr;

pub struct URI {
    scheme: Scheme,
    pub authority: Option<Authority>,
    path: Path,
}

impl URI {
    pub fn new(scheme: Scheme, path: Path) -> URI {
        URI {
            scheme,
            authority: None,
            path,
        }
    }
    pub fn set_authority(&mut self, authority: Authority) {
        self.authority = Some(authority);
    }
}

impl StringRepr for URI {
    fn string_repr(&self) -> String {
        let mut string = format!("{}:", self.scheme.string_repr());
        match &self.authority {
            Some(authority) => string.push_str(format!("//{}", authority.string_repr()).as_ref()),
            None => {}
        }
        string.push_str(self.path.string_repr().as_ref());
        string
    }
}
