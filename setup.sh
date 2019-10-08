#!/bin/sh

DIR=$(dirname "$0")
cd $DIR

echo "create gitignore ..."
echo "# System
.DS_Store
*.swp
*.swo
.vs/

# Rust
/target/
Cargo.lock
**/*.rs.bk

# Godot
.import/
export.cfg
export_presets.cfg
.mono/
data_*/" > .gitignore

echo "clean up ..."
rm -rf ./Cargo.* ./target

echo "cargo init ..."
cargo init

echo "create Rust HelloWorld code ..."
echo 'use gdnative::*;

/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(Node)]
pub struct HelloWorld;

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl HelloWorld {
    
    /// The "constructor" of the class.
    fn _init(_owner: Node) -> Self {
        HelloWorld
    }
    
    // In order to make a method known to Godot, the #[export] attribute has to be used.
    // In Godot script-classes do not actually inherit the parent class.
    // Instead they are"attached" to the parent object, called the "owner".
    // The owner is passed to every single exposed method.
    #[export]
    fn _ready(&self, _owner: Node) {
        // The `godot_print!` macro works like `println!` but prints to the Godot-editor
        // output tab as well.
        godot_print!("hello, world...");
    }
}

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
    handle.add_class::<HelloWorld>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();' > ./src/lib.rs

echo "update Cargo.toml ..."
echo "[package]
name = \"game\"
version = \"0.1.0\"
edition = \"2018\"

[lib]
crate-type = [\"cdylib\"]

[dependencies]
gdnative = { git = \"https://github.com/GodotNativeTools/godot-rust\" }" > Cargo.toml 

echo "create gdnative library ..."
echo "[general]

singleton=false
load_once=true
symbol_prefix=\"godot_\"
reloadable=true

[entry]

OSX.64=\"./target/debug/libgame.dylib\"
X11.64=\"./target/debug/libgame.so\"

[dependencies]

OSX.64=[  ]
X11.64=[  ]" > game_gdnativelibrary.gdnlib

echo "compile ..."
cargo build && echo "Setup Completed."

