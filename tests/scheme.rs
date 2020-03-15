use default_port::DefaultPort;
use string_repr::StringRepr;
use wdg_uri::scheme::Scheme;

#[cfg(test)]
mod scheme {
    use super::{DefaultPort, Scheme, StringRepr};

    mod string_repr {
        use super::{Scheme, StringRepr};

        #[test]
        fn cc16d62c_a8bd_460c_abf8_6dbb17f8e28e() {
            assert_eq!(Scheme::HTTP.string_repr(), "http");
        }

        #[test]
        fn d139ea3e_d3b9_4957_a6e5_48f32b2f34cc() {
            assert_eq!(Scheme::HTTPS.string_repr(), "https");
        }
    }

    mod default_port {
        use super::{DefaultPort, Scheme};

        #[test]
        fn a75ec374_83a2_49ed_b650_ca4e06d3d86f() {
            assert_eq!(Scheme::HTTP.default_port(), Some(80));
        }

        #[test]
        fn ff78d6e1_1027_4606_bd46_f5263dee04ff() {
            assert_eq!(Scheme::HTTPS.default_port(), Some(443));
        }
    }
}
