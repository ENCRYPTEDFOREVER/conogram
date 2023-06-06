use serde::{Deserialize, Deserializer};

pub fn deserialize_boxed_option<'de, D, T>(d: D) -> Result<Option<Box<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    match <Option<T>>::deserialize(d)? {
        Some(object) => Ok(Some(Box::new(object))),
        None => Ok(None),
    }
}

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

pub fn is_false(v: &bool) -> bool {
    !v
}
