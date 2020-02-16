#[cfg(test)]
mod authority {
    use string_repr::StringRepr;
    use wdg_uri::authority::*;
    mod host {
        use super::{Host, StringRepr};
        mod string_repr {
            use super::{Host, StringRepr};
            #[test]
            fn t0() {
                assert_eq!(Host::new("127.0.0.1".into()).string_repr(), "127.0.0.1");
            }
            #[test]
            fn t1() {
                assert_eq!(Host::new("localhost".into()).string_repr(), "localhost");
            }
            #[test]
            fn t2() {
                assert_eq!(Host::new("example.com".into()).string_repr(), "example.com");
            }
        }
    }
}
