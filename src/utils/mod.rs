pub(crate) mod deserialize_utils;

#[macro_export]
///```no_run
///Trait for Type {
///    // Code
///}
///```
macro_rules! impl_trait {
    ($trait: ident for $t: ty {$($impl: tt)*}) => {
        impl $trait for $t {
            $($impl)*
        }
        impl $trait for &$t {
            $($impl)*
        }
        impl $trait for &mut $t {
            $($impl)*
        }
    };
}
