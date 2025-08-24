# gbc
A Gameboy Color emulator in Rust

## Purpose

The purpose of this is to provide a core GBC emulator written in Rust that can run on both macOS and ARM-based embedded systems.

## Compiling GBC Assembly

Install [rgbds](https://rgbds.gbdev.io/)

```bash
brew install rgbds
```

Write your assembly code and compile using:

```bash
rgbasm -o hello-world.o hello-world.asm
rgblink -o hello-world.gb hello-world.o
rgbfix -v -p 0xFF hello-world.gb
```

You're good to go! Now you can write your own GBC code to run in the emulator.
