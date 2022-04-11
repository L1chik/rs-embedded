#![no_std]
#![no_main]

use cortex_m::Peripherals;
use panic_rtt_target as _;

use cortex_m_rt::entry;
use stm32f3xx_hal as hal;

use crate::hal::{delay::Delay, pac, prelude::*, pwm};
use rtt_target::{rprintln, rtt_init_print};
use stm32f3xx_hal::flash;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let mut pac_peripherals = pac::Peripherals::take().unwrap();
    let cortex_peripherals = Peripherals::take().unwrap();

    let mut rcc = pac_peripherals
        .RCC
        .constrain();

    let mut acrt = flash::ACR {_0: ()};
    let clocks = rcc.cfgr.sysclk(48.MHz()).freeze(&mut acrt);

    let mut delay = Delay::new(cortex_peripherals.SYST, clocks);

    loop {

    }
}
