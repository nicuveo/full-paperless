use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaskName {
    #[serde(rename = "consume_file")]
    ConsumeFile,
    #[serde(rename = "train_classifier")]
    TrainClassifier,
    #[serde(rename = "check_sanity")]
    CheckSanity,
    #[serde(rename = "index_optimize")]
    IndexOptimize,
}
