use serde::{Deserialize, Deserializer, Serializer, de::Error};

#[allow(clippy::ref_option)]
pub fn serialize<T, S>(values: &Option<Vec<T>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: std::fmt::Display,
{
    serializer.serialize_str(
        &values
            .as_ref()
            .unwrap()
            .iter()
            .map(T::to_string)
            .collect::<Vec<_>>()
            .join(","),
    )
}

pub fn deserialize<'de, T, D>(de: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    String::deserialize(de)?
        .split(',')
        .map(|s| s.parse().map_err(D::Error::custom))
        .collect::<Result<Vec<_>, _>>()
        .map(Some)
}
