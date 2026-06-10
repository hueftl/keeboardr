# keeboardr
Rust firmware project for the MAKER40 V2 Keyboard from XTIA

## Overview
- Keyboard MCU: STM32F072C8T6 ARM Cortex M0
- Vendor Firmware: VIA
- Test Firmware: VIAL
- Embedded development in Rust with RMK and embassy.dev

## Rust Setup
  - Install Rust toolchain
  - Add target architecture for MCU: `rustup target add thumbv6m-none-eabi`
  - Install Rust tooling needed for development:
    - `cargo install rmkit`
    - `cargo install flip-link` (flip-link for zero-cost stack overflow protection)
    - `cargo install cargo-make` (for build tasks and automating uf2 generation)
    - `curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh` (probe-rs, embedded toolkit)

## Run
- Build the firmware

   ```shell
   cargo build --release
   ```

- Flash using debug probe

   - If you have a debug probe connected to your  board, flashing is quite simple: run the following command to automatically compile and flash RMK firmware to the board:

   ```shell
   cargo run --release
   ```
