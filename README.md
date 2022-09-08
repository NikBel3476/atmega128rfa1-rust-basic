# Basic Rust project for atmega128rfa
## This repository based on [avr-rust/blink](https://github.com/avr-rust/blink)

To build, run:

```bash
rustup override set nightly
```

### Ensure time delays are consistent with a 16MHz microcontroller.
export AVR_CPU_FREQUENCY_HZ=16000000

### Compile the crate to an ELF executable.
cargo build -Z build-std=core --target avr-atmega128rfa1.json --release

