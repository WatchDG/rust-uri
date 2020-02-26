use wdg_uri::authority::{Authority, Host, Port};

#[cfg(test)]
mod authority {
    use super::{Authority, Host, Port};
    mod host {
        use super::Host;
        mod string_repr {
            use super::Host;
            use string_repr::StringRepr;
            #[test]
            fn t0() {
                assert_eq!(Host::new("127.0.0.1").string_repr(), "127.0.0.1");
            }
            #[test]
            fn t1() {
                assert_eq!(Host::new("localhost").string_repr(), "localhost");
            }
            #[test]
            fn t2() {
                assert_eq!(Host::new("example.com").string_repr(), "example.com");
            }
        }
    }
    mod port {
        use super::Port;
        mod string_repr {
            use super::Port;
            use string_repr::StringRepr;
            #[test]
            fn t0() {
                assert_eq!(Port::new("80").string_repr(), "80");
            }
        }
    }
    mod string_repr {
        use super::{Authority, Host, Port};
        use string_repr::StringRepr;
        #[test]
        fn t0() {
            let host = Host::new("localhost");
            let port = Port::new("8080");
            let mut authority = Authority::new(host);
            authority.set_port(port);
            assert_eq!(authority.string_repr(), "localhost:8080");
        }
    }
}
