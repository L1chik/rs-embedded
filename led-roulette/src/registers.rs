#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux5::{entry};
use cortex_m::{iprint, iprintln};


fn main() -> ! {
    aux5::init_led();

    unsafe {
        const GPIOE_BSSR: u32 = 0x48001018;

        *(GPIOE_BSSR as *mut u32) = 1 << 9;

        *(GPIOE_BSSR as *mut u32) = 1 << 11;

        *(GPIOE_BSSR as *mut u32) = 1 << (9 + 16);

        *(GPIOE_BSSR as *mut u32) = 1 << (11 + 16);
    }

    loop {}
}

