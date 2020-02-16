extern crate string_repr;

pub mod authority;
pub mod path;
pub mod query;

use authority::Authority;

pub struct URI {
    pub authority: Authority,
}

impl URI {
    fn new(authority: Authority) -> URI {
        URI { authority }
    }
}
