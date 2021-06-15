#![feature(llvm_asm)]
#![no_main]
#![no_std]
#![feature(abi_avr_interrupt)]

extern crate panic_halt;

extern crate avr_device;
use avr_device::interrupt::Mutex;

extern crate attiny85_hal as hal;
use hal::{port::{portb::{PB0, PB1, PB2, PB3, PB4, PB5}, mode::{Input, Output, PullUp}}};

use hal::prelude::*;

#[cfg(feature = "rt")]
use hal::entry;

use core::cell;
use cell::RefCell;

mod switch;
use switch::Switch;

extern crate embedded_hal;

type BypassSwitch = Switch<PB0<Input<PullUp>>, PB3<Output>, PB2<Output>>;
static BYPASS_SWITCH: Mutex<RefCell<Option<BypassSwitch>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let peripherals = attiny85_hal::pac::Peripherals::take().unwrap();

    // peripherals.EXINT.gimsk.write(|w| w.pcie().set_bit());
    // peripherals.EXINT.pcmsk.write(|w| unsafe { w.bits(0b00000001) });

    let mut portb = peripherals.PORTB.split();
    let bypass_input = portb.pb0.into_pull_up_input(&mut portb.ddr);
    let bypass_output = portb.pb3.into_output(&mut portb.ddr);
    let bypass_led = portb.pb2.into_output(&mut portb.ddr);
    let mut bypass = Switch::new(bypass_input, bypass_output, bypass_led);

    let preset_input = portb.pb1.into_pull_up_input(&mut portb.ddr);
    let preset_output = portb.pb4.into_output(&mut portb.ddr);
    let preset_led = portb.pb5.into_output(&mut portb.ddr);
    let mut preset = Switch::new(preset_input, preset_output, preset_led);

    loop {
        bypass.check();
        preset.check();
    }
}
