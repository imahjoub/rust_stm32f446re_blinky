// std and main are not available for bare metal software
#![no_main]
#![no_std]

use cortex_m_rt::entry; // The runtime
use hal::prelude::*;
use panic_halt as _; // When a panic occurs, stop the microcontroller
use rtt_target::{rprintln as log, rtt_init_print as log_init}; // For logging via SWD debug interface
use stm32f4xx_hal as hal; // needed for the GpioExt trait (-> .split) // When a panic occurs, stop the microcontroller

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
    log_init!();
    if let Some(peripherals) = hal::pac::Peripherals::take() {
        // Set RCC->AHB1ENR GPIOA bit
        let gpioa = peripherals.GPIOA.split();

        // .into_push_pull_output performs three steps
        // 1) set PUPDR: 00 -> no pull-up, no pull-down
        // 2) set OTYPER: 0 -> output push-pull
        // 3) set MODER: 01 -> general purpose output mode
        let mut led = gpioa.pa5.into_push_pull_output();

        loop {
            // toggle pin
            led.toggle();
            #[cfg(debug_assertions)] // Log only in debug builds
            log!("{:?}", led.get_state());
            for _ in 0..50_000 {}
        }
    }

    loop {}
}
