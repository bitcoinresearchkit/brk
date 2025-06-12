use serde::{Deserialize, Deserializer};

pub fn default_on_error<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    match T::deserialize(deserializer) {
        Ok(v) => Ok(v),
        Err(_) => Ok(T::default()),
    }
}
