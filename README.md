# Pong

Pong made using Godot and Rust using the [Godot Rust Bindings](https://github.com/GodotNativeTools/godot-rust). Initial setup done using a shell script from [here](https://gitlab.com/ardawan-opensource/gdnative-rust-setup).
## Quickstart

Follow the instructions
[here](https://medium.com/@recallsingularity/gorgeous-godot-games-in-rust-1867c56045e6).

## Controls

[W][S] and [Up][Down] to move the paddles. [Enter] to hide the left paddle's sprite. [Space] to reset the scene.

## Troubleshooting
If errors are showing up in the Godot editor output, your gdnative paths are
probably messed up. Manually set the paths again and then restart the editor.

If your script properties aren't showing up in the editor, you need to either
restart the editor or run `cargo build` and then restart the editor.

When in doubt, restart the editor.
