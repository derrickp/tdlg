use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ItemRarity {
    Common,
    Exotic,
    Rare,
    Uncommon,
}
