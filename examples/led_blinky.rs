#![deny(unsafe_code)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m;
use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{prelude::*, serial::config::Config, serial::Serial, stm32};

use core::fmt::Write;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioa = dp.GPIOA.split();
        let mut led = gpioa.pa5.into_push_pull_output();

        let tx_pin = gpioa.pa2.into_alternate_af7();
        let rx_pin = gpioa.pa3.into_alternate_af7();

        // Set up the system clock. We want to run at 84MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(84.mhz()).freeze();

        let serial = Serial::usart2(
            dp.USART2,
            (tx_pin, rx_pin),
            Config::default().baudrate(115_200.bps()),
            clocks,
        )
        .unwrap();

        let (mut tx, mut _rx) = serial.split();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        loop {
            // On for 1s, off for 1s.
            if led.set_high().is_err() {
                // Handle error?
            }
            delay.delay_ms(1000_u32);
            if led.set_low().is_err() {
                // Handle error?
            }
            delay.delay_ms(1000_u32);
            writeln!(tx, "Hello, world!\r",).unwrap();
        }
    }

    loop {}
}
