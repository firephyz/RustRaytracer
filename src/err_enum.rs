use std::fmt;
use std::convert::From;

macro_rules! ErrorEnum {
    ( $enum_name:ident, ($($error_type:ident),*)) => (
        #[derive(Debug)]
        enum $enum_name {$(
            $error_type($error_type),
        )*}

        impl fmt::Display for $enum_name {
            fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                match self {
                    $(
                        Self::$error_type(e) => write!(formatter, "{}", e).unwrap(),
                    )*
                };
                Ok(())
            }
        }

        $(
            impl From<$error_type> for $enum_name {
                fn from(err: $error_type) -> Self {
                    $enum_name::$error_type(err)
                }
            }
        )*
    );
}
