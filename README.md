# tdlg

**Note:** This is something I've been working on while learning Rust and attempting to learn game dev. I do not guarantee it is good at all. It is also not something that has been used in a real game.

This library generates a top down map grid based on "stamping" template rooms onto the map in random locations. The templates can be defined by calling applications using text files in nested folders.

## Template files
Templates are divided into 2 parts. There are the base templates which define the different configurations for the room (different wall placements, door placements, destroyed walls, etc.) and then "fill" templates which are combined with each of the base templates. These "fill" templates would define items and other pieces that would go into the room (such as a table). The different possible layer types are `src/cells/layer.rs`. Some example templates can be found in a small game I'm toying around with at (https://github.com/derrickp/under_farm/tree/main/assets/room_templates).

You do not need item configuration to use the generator, and any basic room templates will do.
