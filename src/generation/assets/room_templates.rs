use std::num::NonZeroU8;

use crate::map::{layers::LayerType, Room};

const FOUR_BY_FOUR_BROKEN: &str = r#"
||&&&|
|====|
|====|
|====|
|====|
||||||
"#;

const OTHER_ONE: &str = r#"
||||||||||
|========|
|========|
|=======||
|======||
|======|
|====|||
|====|
|===||
|||||
"#;

const OTHER_TWO: &str = r#"
||||||||||||||||
|==============|
|=============|
|==========||||
|========|||
|========|
|======|||
|======|
|====|||
|====|
|===||
|===|
|==||
|==|
|==|
||||
"#;

const OTHER_THREE: &str = r#"
||||||
|====|
|====|
|===||
|==||
|==|
||||
"#;

const OTHER_FOUR: &str = r#"
|||||
|===
||
|
"#;

fn create_square_room_template_text(floor_count: NonZeroU8) -> String {
    let max = floor_count.get() + 1;

    let mut text = String::new();

    for y in 0..=max {
        for x in 0..=max {
            if x > 0 && x < max && y > 0 && y < max {
                text.push(LayerType::RoomFloor.into());
            } else {
                text.push(LayerType::RoomWall.into());
            }
        }
        text.push('\n');
    }

    text
}

pub fn all_room_templates() -> Vec<Room> {
    vec![
        create_square_room_template_text(NonZeroU8::new(2).unwrap()).into(),
        create_square_room_template_text(NonZeroU8::new(3).unwrap()).into(),
        create_square_room_template_text(NonZeroU8::new(4).unwrap()).into(),
        create_square_room_template_text(NonZeroU8::new(5).unwrap()).into(),
        FOUR_BY_FOUR_BROKEN.trim().into(),
        OTHER_ONE.trim().into(),
        OTHER_TWO.trim().into(),
        OTHER_THREE.trim().into(),
        OTHER_FOUR.trim().into(),
    ]
}

#[derive(Clone, Debug)]
pub struct RoomTemplates {
    pub rooms: Vec<Room>,
}

impl Default for RoomTemplates {
    fn default() -> Self {
        Self {
            rooms: all_room_templates(),
        }
    }
}
