#![no_std]
#![no_main]

use cortex_m_rt::entry;
use stm32f4xx_hal::{pac, prelude::*};
#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller

// This marks the entrypoint of our application. The cortex_m_rt creates some
// startup code before this, but we don't need to worry about this
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpioc = dp.GPIOC.split();
    let gpioa = dp.GPIOA.split();

    // Push pull configuration: https://open4tech.com/open-drain-output-vs-push-pull-output/
    // When set to 0, led will be on; and when set to 1 it will be set to 0
    let mut led = gpioc.pc13.into_push_pull_output();
    let key_btn = gpioa.pa0.into_pull_up_input();

    loop {
        if key_btn.is_high() {
            led.set_high();
        } else {
            led.set_low();
        }
    }
}
