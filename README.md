# keeboardr
Rust firmware project for the MAKER40 V2 Keyboard from XTIA

## Overview
- Keyboard MCU: STM32F072C8T6 ARM Cortex M0
- Vendor Firmware: VIA
- Test Firmware: VIAL

## Rust Setup
  - Install Rust toolchain
  - Add target architecture for MCU: `rustup target add thumbv6m-none-eabi`
  - Install Rust tooling needed for development:
    - `cargo install cargo-binutils`
    - `rustup component add llvm-tools`
    - `cargo install cargo-generate` (for creating a project from template)
    -
