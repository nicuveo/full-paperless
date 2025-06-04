use serde_repr::{Deserialize_repr, Serialize_repr};

#[repr(i8)]
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr,
)]
pub enum AssignCorrespondentFrom {
    DoNotAssignACorrespondent = 1,
    UseMailAddress = 2,
    UseNameOrMailAddress = 3,
    UseGivenCorrespondent = 4,
}
