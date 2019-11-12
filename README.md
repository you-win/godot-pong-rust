# Pong

Pong made using Godot and Rust using the [Godot Rust Bindings](https://github.com/GodotNativeTools/godot-rust). Initial setup done using a shell script from [here](https://gitlab.com/ardawan-opensource/gdnative-rust-setup).

I posted a writeup on Reddit [here](https://www.reddit.com/r/godot/comments/dfam0p/i_made_a_pong_clone_in_godot_using_the_gdnative/).

## Quickstart

Follow the instructions
[here](https://medium.com/@recallsingularity/gorgeous-godot-games-in-rust-1867c56045e6).

You will need to set up the .gdnlib paths again during initial setup. This is
both to avoid errors and because the path is only setup for OSX.

If you are using VSCode along with the Rust extension, you will need to place a `.gdignore` file in the generated rls/ directory. If you don't, Godot will try to import everything in that directory and subsequently hang.

Run the default scene to play.

## Controls

[W][S] and [Up][Down] to move the paddles. [Enter] to hide the left paddle's sprite. [Space] to reset the scene.

## Troubleshooting
If errors are showing up in the Godot editor output, your gdnative paths are
probably messed up. Manually set the paths again and then restart the editor.

If your script properties aren't showing up in the editor, you need to either
restart the editor or run `cargo build` and then restart the editor.

When in doubt, restart the editor.
