pub mod map {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::collections::HashMap;

    use crate::schema::model;

    pub fn serialize<S>(
        values: &HashMap<i32, serde_json::Value>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        values
            .iter()
            .map(|(field, value)| model::CustomFieldInstance {
                field: *field,
                value: value.clone(),
            })
            .collect::<Vec<model::CustomFieldInstance>>()
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(de: D) -> Result<HashMap<i32, serde_json::Value>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Vec::<model::CustomFieldInstance>::deserialize(de)?
            .into_iter()
            .map(|cfi| (cfi.field, cfi.value))
            .collect())
    }
}

pub mod map_option {
    use serde::{Deserializer, Serializer};
    use std::collections::HashMap;

    #[allow(clippy::ref_option)]
    pub fn serialize<S>(
        values: &Option<HashMap<i32, serde_json::Value>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(values) = values.as_ref() {
            super::map::serialize(values, serializer)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(de: D) -> Result<Option<HashMap<i32, serde_json::Value>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        super::map::deserialize(de).map(Some)
    }
}
