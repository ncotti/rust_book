//! Demonstrate the use of a blocking `Delay` using the SYST (sysclock) timer.
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use stm32f4::stm32f401;
use cortex_m_rt::entry;
use cortex_m;

// Led is on PA5.


#[entry]
fn main() -> ! {
    let p = stm32f401::Peripherals::take().unwrap();
    let mut core_p = cortex_m::Peripherals::take().unwrap();
    systick_init(&mut core_p.SYST);
    led_init(&p.GPIOA, &p.RCC);

    loop {
        p.GPIOA.odr.modify(|r, w| w.odr5().bit(!r.odr5().bit()));
        while !core_p.SYST.has_wrapped() {
            cortex_m::asm::nop();
        }
    }
}

fn systick_init(systick: &mut cortex_m::peripheral::SYST) {
    systick.set_reload(16_000_000);
    systick.clear_current();
    systick.set_clock_source(cortex_m::peripheral::syst::SystClkSource::Core);
    systick.disable_interrupt();
    systick.enable_counter();
}

fn led_init(porta: &stm32f401::GPIOA, rcc: &stm32f401::RCC) {
    rcc.ahb1enr.write(|w| w.gpioaen().enabled());
    porta.moder.write(|w| w.moder5().output() );
    porta.otyper.write(|w| w.ot5().push_pull() );
    porta.odr.write(|w| w.odr5().high());
}