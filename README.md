# tdlg

**Note:** This is something I've been working on while learning Rust and attempting to learn game dev. I do not guarantee it is good at all. It is also not something that has been used in a real game.

This library generates a top down map grid based on "stamping" template rooms onto the map in random locations. The templates can be defined by using creating a `Room` from a string.

## Template files
Templates should be looked at as defining an "empty" room. During the move to 2.0 I removed the "fill" templates as it was making it much more complicated. Eventually I am going to add more to the generation to allow for specifying generation of tables and other structures inside of the rooms before they are stamped into the grid.

You can example of the usage inside of the examples, or inside of https://github.com/derrickp/under_farm
