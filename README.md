# STM32-SHT31
A Rust implementation for driving the SHT31 using STM32
## Requirement
MCU: STM32F411CEU6\
Rust: rust nightly
## Compile
```
RUSTFLAGS="-C link-arg=-Tlink.x" cargo build --target thumbv7em-none-eabihf
```
