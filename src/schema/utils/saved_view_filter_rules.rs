use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::schema::model;

pub fn serialize<S>(values: &[(model::RuleType, String)], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    values
        .iter()
        .map(|(k, v)| model::SavedViewFilterRule {
            rule_type: *k,
            value: v.clone(),
        })
        .collect::<Vec<model::SavedViewFilterRule>>()
        .serialize(serializer)
}

pub fn deserialize<'de, D>(de: D) -> Result<Vec<(model::RuleType, String)>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Vec::<model::SavedViewFilterRule>::deserialize(de)?;
    Ok(value
        .into_iter()
        .map(|fr| (fr.rule_type, fr.value))
        .collect())
}
