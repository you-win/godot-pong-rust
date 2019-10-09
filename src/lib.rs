#[macro_use]
extern crate gdnative as godot;

extern crate rand;

mod ball;
mod hello_world;
mod paddle;
mod utils;

use self::{ball::Ball, hello_world::HelloWorld, paddle::Paddle};

// Function that registers all exposed classes to Godot
fn init(handle: godot::init::InitHandle) {
    handle.add_class::<HelloWorld>();
    handle.add_class::<Paddle>();
    handle.add_class::<Ball>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();
