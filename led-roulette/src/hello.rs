#![deny(unsafe_code)]
#![no_main]
#![no_std]


#[allow(unused_imports)]
use cortex_m::{iprint, iprintln};
use aux5::entry;

// #[entry]
// fn main() -> ! {
//     let mut itm = aux5::init_itm();
//
//     iprintln!(&mut itm.stim[0], "Hello, world!");
//
//     loop {}
//
// }


#[entry]
fn main_panic() -> ! {
    panic!("Im done");
}