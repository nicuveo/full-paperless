use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(u8)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum MatchingAlgorithm {
    None = 0,
    AnyWord = 1,
    AllWords = 2,
    ExactMatch = 3,
    RegularExpression = 4,
    FuzzyWord = 5,
    Automatic = 6,
}
