rust_stm32f446re_blinky
=============================

<p align="center">
    <a href="https://github.com/imahjoub/rust_stm32f446re_blinky/actions">
        <img src="https://github.com/imahjoub/rust_stm32f446re_blinky/actions/workflows/rust_stm32f446re_blinky.yml/badge.svg" alt="Build Status"></a>
    <a href="https://github.com/imahjoub/rust_stm32f446re_blinky/issues?q=is%3Aissue+is%3Aopen+sort%3Aupdated-desc">
        <img src="https://custom-icon-badges.herokuapp.com/github/issues-raw/imahjoub/rust_stm32f446re_blinky?logo=github" alt="Issues" /></a>
    <a href="https://github.com/imahjoub/rust_stm32f446re_blinky" alt="GitHub code size in bytes">
        <img src="https://img.shields.io/github/languages/code-size/imahjoub/rust_stm32f446re_blinky" /></a>
    <a href="https://github.com/imahjoub/rust_stm32f446re_blinky/blob/main/LICENSE_1_0.txt">
        <img src="https://img.shields.io/badge/license-BSL%201.0-blue.svg" alt="Boost Software License 1.0"></a>
    <a href="https://github.com/imahjoub/rust_stm32f446re_blinky" alt="Activity">
        <img src="https://img.shields.io/github/commit-activity/y/imahjoub/rust_stm32f446re_blinky" /></a>
</p>


- A mini application to demonstrate embedded development using Rust. Created using a `NUCLEO-F446RE development board`. Blinks an LED on pin `PA5`.

- The application uses Visual Studio as IDE to build source files and flash the board.

- The repository provides a compressed cargo package located in `rust_stm32f446re_blinky\toolchain\cargo\bin.7z`.
Unpack the cargo package before building the application.


## Using rust_stm32f446re_blinky with visual Studio (Windows)

### Required tools
  - Visual Studio

### cross compilation
  - After unzipping the bin folder, open a CMD window in that folder and run the following commands
  ``
  - ustup target add thumbv7em-none-eabihf
  - argo install cargo-embed cargo-binutils
  ``

### Build the rust_stm32f446re_blinky application
  - Open the solution `rust_stm32f446re_blinky.sln` in the `./rust_stm32f446re_blinky directory`.
  - Select the `stm32f446re_blinky_build` configuration.
  - Then rebuild the entire solution.

### Flash NUCLEO-F446RE with rust_stm32f446re_blinky
  - connect the NUCLEO-F446RE boards to your pc
  - Select the `stm32f446re_blinky_flash` configuration.
  - Then rebuild the entire solution.

## Using rust_stm32f446re_blinky in Linux

### Cross-compilation installation
  ``
  - curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  - rustup target add thumbv7em-none-eabihf
  - cargo install cargo-embed cargo-binutils
  ``

## Build application
  ``
  - cargo build
  ``

## flash target (NUCLEO-F446RE)
  ``
  - cargo embed
  ``
