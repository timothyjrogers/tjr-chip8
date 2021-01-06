# Rusty Chip8

This is a straight forward Chip-8 emulator written in Rust. I am a relatively new Rust user so feedback on the emulator itself or my Rust code is welcome.

To build and run the emulator issue the following command:

```bash
cargo run --release
```
Note that you will need to source ROM files from the internet and you must acquire the appropriate SDL2 DLL for your system and place it beside the emulator binary -- see  [rust-sdl2 on Github](https://github.com/Rust-SDL2/rust-sdl2) for more details.

## Dependencies
The following crates are imported (per Cargo.toml) for the following purposes:

* sdl2 - SDL2 bindings used to create and draw emulator window, receive keyboard input, output sound
* rand - Used for random number generation

## Organization
This project is split into the following files:

* main.rs - This is the entry point for the application; it instantiates the various components and maintains the CPU clock loop.
* cpu.rs - The CPU contains implementation for the fetch/decode/execute 'tick' as well as each opcode.
* screen.rs - The Screen contains data structures and methods for creating the game window and drawing to the screen as part of the DYXN operation.
* keypad.rs - The Keypad contains data structures and methods for receiving input from the 16-key keypad and checking the keypad state.
* audio.rs - The Audio module contains data structures and methods for generating the single-tone sound output.
