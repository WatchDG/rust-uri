extern crate string_repr;

pub mod authority;
pub mod path;
pub mod query;
pub mod scheme;

use authority::Authority;
use scheme::Scheme;

pub struct URI {
    scheme: Scheme,
    pub authority: Option<Authority>,
}

impl URI {
    fn new(scheme: Scheme) -> URI {
        URI {
            scheme,
            authority: None,
        }
    }
}
