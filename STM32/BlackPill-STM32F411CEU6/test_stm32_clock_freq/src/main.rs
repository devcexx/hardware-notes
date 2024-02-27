#![no_std]
#![no_main]

use core::arch::asm;

use cortex_m::delay::Delay;
use cortex_m_rt::entry;

#[allow(unused_imports)]
use panic_halt;
use stm32f4xx_hal::gpio::GpioExt;
use stm32f4xx_hal::pac;


#[inline(always)]
fn delay4() {
    // From documentation:
    /*
    A delay between an RCC peripheral clock enable and the effective peripheral
    enabling should be taken into account in order to manage the peripheral read/write
    from/to registers.
      (+) This delay depends on the peripheral mapping.
      (+) If peripheral is mapped on AHB: the delay is 2 AHB clock cycle
          after the clock enable bit is set on the hardware register
      (+) If peripheral is mapped on APB: the delay is 2 APB clock cycle
    after the clock enable bit is set on the hardware register
     */

    // Just putting a couple of nops here for a quick delay
    unsafe {
        asm! {
            "nop",
            "nop",
            "nop",
            "nop"
        }
    }
}

/*
 * On this example I'm manually modifying the registers for setting up
 * the speed of the different clocks of the STM32.
 *
 * Refer to section 6.2 in STM31F411xC/E advanced manual for more information
 *
 * The example uses some parameters to target the following rates:
 * SYSCLK: 100 MHz
 * AHB: 100 MHz
 * APB1: 50 MHz
 * APB2: 50 MHz
 * TIM1: APB1 * 2 = 100 MHz
 * TIM2: APB2 * 2 = 100 MHz
 *
 * For that we're using the following calculations:
 *
 * HSE (25 MHz) / PLLM (25) * PLLN (200) / PLLP (2) = 100 MHz
 *
*/
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cortex = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC;
    let pwr = dp.PWR;

    let gpioc = dp.GPIOC.split();
    let gpioa = dp.GPIOA.split();
    let mut led = gpioc.pc13.into_push_pull_output();
    led.set_high();

    // Configure the main internal regulator output voltage
    rcc.apb1enr.modify(|_, x| x.pwren().set_bit());

    // Technically this is required for keeping the device up with a clock of 100 MHz
    pwr.cr.modify(|_, x| x.vos().variant(0b11)); // Scale 1 <= 100 MHz

    // It is expected to have a delay after a RCC peripheral clock
    // enable.  Is the modification of the previous register doing
    // that? Not fully sure about this. If you remove this, everything
    // will still working tho.
    delay4();

    // Enable HSE
    rcc.cr.modify(|_, x| x.hseon().on());
    while rcc.cr.read().hserdy().bit_is_clear() {} // Wait for HSE to be enabled

    // Prepare PLL, disable it first
    rcc.cr.modify(|_, x| x.pllon().off());
    while rcc.cr.read().pllrdy().bit_is_set() {} // Wait for PLL to be disabled

    // Configure PLL params
    rcc.pllcfgr.modify(|_, x| x
        .pllsrc().hse()
        .pllm().variant(25)
        .plln().variant(200)
        .pllp().div2()
        .pllq().variant(4)
    );

    // Enable PLL
    rcc.cr.modify(|_, w| w.pllon().on());
    while rcc.cr.read().pllrdy().bit_is_clear() {} // Wait for PLL to be enabled

    led.set_low();

    // TODO Set flash latency?

    rcc.cfgr.modify(|_, x| x
        .sw().pll()
        .hpre().div1()
        .ppre1().div2()
        .ppre2().div2()
    );

    // Test the new AHB frequency (that will match SYSCLK because AHB prescaler is /1)
    // by creating a delay with the expected frequency. If the observed period of
    // led blinking is 1 second, then the configuration is correct.
    let mut delay = Delay::new(cortex.SYST, 100_000_000);

    loop {
        led.set_low();
        delay.delay_ms(1000);

        led.set_high();
        delay.delay_ms(1000);
    }
}
