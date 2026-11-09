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

pub trait ToHex {
    fn hex(&self) -> String;
}

impl<T: AsRef<[u8]>> ToHex for T {
    fn hex(&self) -> String {
        const HEX: &[u8; 16] = b"0123456789abcdef";

        let v = self.as_ref();
        let mut bytes = Vec::with_capacity(v.len() * 2);
        for &b in v {
            bytes.push(HEX[(b >> 4) as usize]);
            bytes.push(HEX[(b & 0xf) as usize]);
        }

        // SAFETY: Hex characters (0-9, a-f) are valid UTF-8
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}

pub(crate) fn sha2_digest(data: impl AsRef<[u8]>) -> String {
    let digest = sha2::Sha256::digest(data);
    digest.hex()
}
