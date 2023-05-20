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



Using Rust to turn on an LED on the NUCLEO-F446RE by pressing a button

This example is mainly copied from the great Rust Embedded [Cortex-M Quickstart](https://github.com/rust-embedded/cortex-m-quickstart)
and Jesse Braham's article [Embedded Rust: From Zero to Blinky](https://beta7.io/posts/embedded-rust-from-zero-to-blinky.html)
and tweaked for the STM32F446RE of the NUCLEO-F446RE board.

prerequisites: rust, opencd

0. `rustup target add thumbv7em-none-eabihf`
1. `git clone https://github.com/krenzlin/rust-stm32f446-blinky`
2. `cd rust-stm32f466-blinky`
3. `cargo build`
4. `sudo ./flash_device.sh target/thumbv7em-none-eabihf/debug/blinky`
5. press the blue user button to turn on the LED

Troubleshooting:

* try updating: `rustup update`

