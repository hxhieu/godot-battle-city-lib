use gdnative::prelude::*;
mod player;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<player::main::Player>();
}

// Macro that creates the entry-points of the dynamic library.
godot_init!(init);
