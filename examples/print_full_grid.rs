use std::num::NonZeroU16;

use pathfinding::prelude::astar;
use tdlg::generation::Generator;

fn main() {
    let generator = Generator::build(
        "godzilla",
        NonZeroU16::new(100).unwrap(),
        NonZeroU16::new(30).unwrap(),
    );

    let map = generator.generate_top_down_map().unwrap();

    let _ = astar(
        map.entry(),
        |c| {
            map.grid()
                .surrounding_walkable_coordinates(c)
                .into_iter()
                .map(|c| (c, 1))
        },
        |c| c.distance(map.exit()) / 3,
        |c| c.eq(map.exit()),
    );

    let layer_display = map.grid().top_layer_display();
    std::fs::write("./example_output/grid.txt", layer_display).unwrap();
}
