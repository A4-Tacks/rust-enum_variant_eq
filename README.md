Determining that enumerations only have Variant equal

# Examples
```rust
use enum_variant_eq::{*,enum_variant_eq_derive::*};

#[derive(Debug, EnumVariantEq)]
enum TestEnum {
    Unit,
    EmptyUnNamed(),
    UnNamed(i8),
    UnNamed2(i8, u8),
    EmptyNamed {},
    Named { x: i8 },
    Named2 { x: i8, y: u8 },
}
use TestEnum::*;
assert!(Unit.enum_variant_eq(&Unit));
assert!(UnNamed(5).enum_variant_eq(&UnNamed(8)));
assert!(Named { x: 5 }.enum_variant_eq(&Named { x: 8 }));
```

# Errors
`EnumVariantEq` only applies to enumerations. If it is violated, the compilation will panic
```rust
#[derive(Debug, EnumVariantEq)]
struct S;
```
```
error: #[derive(Debug, EnumVariantEq)]
    |                  ^^^^^^^^^^^^^
    = help: message: Type Is Not Enum
```

# Other:
`Build Ast Error`
