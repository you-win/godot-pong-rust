use gdnative::prelude::*;

extern crate rand;

mod ball;
mod paddle;
mod utils;

use self::{ball::Ball, paddle::Paddle};

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<Paddle>();
    handle.add_class::<Ball>();
}

// macros that create the entry-points of the dynamic library.
godot_init!(init);
