Playground for learning how to use https://www.waveshare.com/wiki/RP2040-LCD-1.28 with rust.

## Prerequisites

Install [elf2uf2-rs](https://crates.io/crates/elf2uf2-rs)

    cargo install elf2uf2-rs

## Compile

Compile code and transfer to the device

    cargo run --example text --release
