#![deny(unsafe_code)]
#![no_main]
#![no_std]
use aux5::{Delay, DelayMs, entry, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let half_period = 500_u16;

    loop {
        for curr in 0..leds.len() {
            let next = (curr + 1) % 8;

            leds[next].on().ok();
            delay.delay_ms(half_period);

            leds[curr].off().ok();
            delay.delay_ms(half_period);
        }

    }
}
