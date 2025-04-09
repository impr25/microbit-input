# Micro:bit Input Project 

##readme written by CoPilot

This project demonstrates button input handling and LED matrix control on a BBC micro:bit v2, with serial communication capabilities.

## Features

- Button A and B input detection
- Full LED matrix display feedback on button press
- Serial communication (115200 baud rate)
  - Sends 'i' when Button A is pressed
  - Sends 'j' when Button B is pressed

## Prerequisites

- Rust toolchain
- `thumbv7em-none-eabihf` target: `rustup target add thumbv7em-none-eabihf`
- probe-rs: `cargo install probe-rs-cli`
- BBC micro:bit v2 board

## Setup

1. Clone the repository
2. Connect your micro:bit v2 to your computer via USB
3. Build and flash the project:
```bash
cargo run
```

## Project Structure

- `src/main.rs` - Main application code handling button inputs and LED display
- `src/serial.rs` - UART communication implementation
- `.cargo/config.toml` - Cargo configuration for cross-compilation
- `Embed.toml` - probe-rs configuration for flashing and debugging

## Serial Protocol

The device communicates over UART with the following settings:
- Baud Rate: 115200
- Data Bits: 8
- Parity: None
- Stop Bits: 1

Output Format:
- 'i' - Button A pressed
- 'j' - Button B pressed

## License

MIT License

## Contributing

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.
