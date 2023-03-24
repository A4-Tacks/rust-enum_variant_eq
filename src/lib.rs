pub use enum_variant_eq_derive;

/// Determining whether the enumeration Variant are equal
/// # Examples
/// ```
/// use enum_variant_eq::{*,enum_variant_eq_derive::*};
///
/// #[derive(Debug, EnumVariantEq)]
/// enum TestEnum {
///     Unit,
///     EmptyUnNamed(),
///     UnNamed(i8),
///     UnNamed2(i8, u8),
///     EmptyNamed {},
///     Named { x: i8 },
///     Named2 { x: i8, y: u8 },
/// }
/// # use TestEnum::*;
/// assert!(Unit.enum_variant_eq(&Unit));
/// assert!(UnNamed(5).enum_variant_eq(&UnNamed(8)));
/// assert!(Named { x: 5 }.enum_variant_eq(&Named { x: 8 }));
/// #
/// # assert!(UnNamed(5).enum_variant_eq(&UnNamed(5)));
///
/// # assert!(EmptyUnNamed().enum_variant_eq(&EmptyUnNamed()));
/// # assert!(EmptyNamed {}.enum_variant_eq(&EmptyNamed {}));
/// # assert!(UnNamed2(5, 6).enum_variant_eq(&UnNamed2(8, 9)));
/// # assert!(Named2 { x: 5, y: 6 }.enum_variant_eq(&Named2 { x: 8, y: 9 }));
///
/// ```
pub trait EnumVariantEq {
    fn enum_variant_eq(&self, other: &Self) -> bool;
}


#[cfg(test)]
mod tests {
    use super::EnumVariantEq;
    use super::enum_variant_eq_derive::EnumVariantEq;


    #[derive(Debug, EnumVariantEq)]
    #[allow(unused)]
    enum EmptyEnum {}

    #[derive(Debug, EnumVariantEq)]
    #[allow(unused)]
    #[repr(usize)]
    enum TestEnum {
        Unit = 8,
        EmptyUnNamed() = 9,
        UnNamed(i8) = 15,
        UnNamed2(i8, u8),
        EmptyNamed {} = 19,
        Named { x: i8 },
        Named2 { x: i8, y: u8 },
    }

    #[allow(unused)]
    macro_rules! asserts {
        (! $( $a:expr, $b:expr ; )* ) => {
            $(
                assert!( ! $a.enum_variant_eq(&$b) );
            )*
        };
        ( $( $a:expr, $b:expr ; )* ) => {
            $(
                assert!( $a.enum_variant_eq(&$b) );
            )*
        };
    }

    #[test]
    fn tests() {
        // TestEnum
        {
            use TestEnum::*;
            // Assert eq
            asserts!{
                Unit, Unit;
                EmptyUnNamed(), EmptyUnNamed();
                UnNamed(3), UnNamed(4);
                UnNamed2(3, 5), UnNamed2(4, 6);
                UnNamed2(3, 5), UnNamed2(3, 5);
                EmptyNamed {}, EmptyNamed {};
                Named { x: 3 }, Named { x: 4 };
                Named2 { x: 6, y: 8 }, Named2 { x: 7, y: 9 };
                Named2 { x: 7, y: 8 }, Named2 { x: 7, y: 8 };
            }
            // Assert ne
            asserts!{!
                Unit, EmptyUnNamed();
                EmptyUnNamed(), UnNamed(7);
                UnNamed(8), UnNamed2(12, 16);
                UnNamed2(17, 18), EmptyNamed {};
                EmptyNamed {}, Named { x: 8 };
                Named { x: 9 }, Named2 { x: 12, y: 15 };
                Named2 { x: 3, y: 8 }, Unit;
            }
        }
    }
    /// Generic or LifeTime Test
    #[test]
    fn generic_test() {
        #[derive(Debug, EnumVariantEq)]
        enum Test<'a, T> {
            A(&'a T),
            No,
        }
        use Test::*;
        assert!(A(&Vec::new()).enum_variant_eq(&A(&vec![1, 2])));
        assert!(No::<()>.enum_variant_eq(&No));
    }
}
