# Rusty Chip8

This is a straight forward Chip-8 emulator written in Rust. Code quality, style, etc. should improve over time as I'm using this to brush up on my Rust skills.

To run the emulator issue the following:

```bash
cargo run /full/path/to/rom clock_speed_hz
```

Note that you will need to source  ROM files from the internet and you must acquire the appropriate SDL2 DLL for your system and place it beside the emulator executable.