#![deny(unsafe_code)]
// std and main are not available for bare metal software
#![no_main]
#![no_std]

use cortex_m_rt::entry;       // The runtime
use stm32f4xx_hal as hal;
use hal::prelude::*;          // needed for the GpioExt trait (-> .split)
#[allow(unused)]
use panic_halt;               // When a panic occurs, stop the microcontroller

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
    if let Some(peripherals) = hal::stm32::Peripherals::take() {
        let gpioa = peripherals.GPIOA.split(); // + sets RCC->AHB1ENR GPIOA bit

        // .into_push_pull_output performs three steps
        // 1) set PUPDR: 00 -> no pull-up, no pull-down
        // 2) set OTYPER: 0 -> output push-pull
        // 3) set MODER: 01 -> general purpose output mode
        let mut led = gpioa.pa5.into_push_pull_output();

        let gpioc = peripherals.GPIOC.split();
        let button = gpioc.pc13; // pins are input by default

        loop {
            // .is_high reads IDR
            if button.is_high().unwrap() {
                // .set_low uses BSRR
                led.set_low().unwrap();
            } else {
                led.set_high().unwrap();
            }
        }
    }
    loop {}
}
