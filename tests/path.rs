use string_repr::StringRepr;
use wdg_uri::path::Path;

#[cfg(test)]
mod path {
    use super::{Path, StringRepr};

    mod string_repr {
        use super::{Path, StringRepr};

        #[test]
        fn fc0f0dd2_530d_4e38_8d33_f9985ee0f848() {
            assert_eq!(
                Path::new("/path/to/something").string_repr(),
                "/path/to/something"
            )
        }
    }
}
