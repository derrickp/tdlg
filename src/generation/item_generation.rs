use std::ops::Range;

use crate::map::cells::LayerType;

#[derive(Clone)]
pub struct ItemChance {
    pub layer_type: LayerType,
    pub chance: Range<usize>,
}

#[derive(Clone)]
pub struct ItemGeneration {
    pub target_num_items: usize,
    pub item_ranges: Vec<ItemChance>,
}
