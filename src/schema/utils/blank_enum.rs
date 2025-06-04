use serde::de::IntoDeserializer;
use serde::Deserialize;

pub fn blank_enum<'de, T, D>(de: D) -> Result<Option<T>, D::Error>
where
    T: serde::Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    match Option::<String>::deserialize(de)?.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => T::deserialize(s.into_deserializer()).map(Some),
    }
}
