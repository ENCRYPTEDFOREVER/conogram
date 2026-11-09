use std::fmt::Write as _;

use sha2::Digest as _;

pub(crate) mod deserialize_utils;

#[macro_export]
///```ignore
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

pub(crate) fn sha2_digest(data: impl AsRef<[u8]>) -> String {
    let digest = sha2::Sha256::digest(data);
    let mut hash = String::with_capacity(64);
    for b in digest {
        let _ = write!(&mut hash, "{b:02x}");
    }
    hash
}
