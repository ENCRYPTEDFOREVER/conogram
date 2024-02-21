use serde::{Deserialize, Deserializer};

pub fn deserialize_boxed<'de, D, T>(d: D) -> Result<Box<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    match <T>::deserialize(d) {
        Ok(value) => Ok(Box::new(value)),
        Err(err) => Err(err),
    }
}

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn is_false(v: &bool) -> bool {
    !v
}
