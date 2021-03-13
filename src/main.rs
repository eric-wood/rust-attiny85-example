#![feature(llvm_asm)]
#![no_main]
#![no_std]

extern crate panic_halt;

extern crate avr_device;

extern crate attiny85_hal as hal;

use hal::prelude::*;

#[cfg(feature = "rt")]
use hal::entry;

type Delay = hal::delay::Delay<hal::clock::MHz8>;

#[entry]
fn main() -> ! {
    let mut delay = Delay::new();

    let dp = attiny85_hal::pac::Peripherals::take().unwrap();
    let mut portb = dp.PORTB.split();
    let mut pb1 = portb.pb1.into_output(&mut portb.ddr);

    loop {
        pb1.toggle().void_unwrap();
        delay.delay_us(500u16);
    }
}
