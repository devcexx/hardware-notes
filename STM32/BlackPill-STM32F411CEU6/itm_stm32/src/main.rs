#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::{delay::Delay, iprintln};
use stm32f4xx_hal::{pac, prelude::*};
#[allow(unused_imports)]
use panic_halt; // When a panic occurs, stop the microcontroller

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cortex = cortex_m::Peripherals::take().unwrap();

    let gpioc = dp.GPIOC.split();
    let rcc = dp.RCC.constrain();

    let _ = rcc
        .cfgr
        .use_hse(25.MHz())
        .sysclk(100.MHz())
        .freeze();

    // Push pull configuration: https://open4tech.com/open-drain-output-vs-push-pull-output/
    // When set to 0, led will be on; and when set to 1 it will be set to 0
    let mut led = gpioc.pc13.into_push_pull_output();

    let mut delay = Delay::new(cortex.SYST, 100_000_000);

    let mut itm = cortex.ITM;
    let stim = &mut itm.stim[0];

    let mut counter = 0;
    loop {
        iprintln!(stim, "Counter is: {}", counter);
        led.toggle();
        delay.delay_ms(1000);
        counter += 1;
    }
}
