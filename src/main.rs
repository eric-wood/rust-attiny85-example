#![feature(llvm_asm)]
#![no_main]
#![no_std]
#![feature(abi_avr_interrupt)]

extern crate panic_halt;

extern crate avr_device;
use avr_device::interrupt;
use avr_device::interrupt::{free, Mutex};

extern crate attiny85_hal as hal;
use hal::{
    port::{
        mode::{Input, Output, PullUp},
        portb::{PB0, PB1, PB2, PB3, PB4, PB5},
    },
};

use hal::prelude::*;

#[cfg(feature = "rt")]
use hal::entry;

use cell::RefCell;
use core::cell;

mod switch;
use switch::Switch;

mod timer;
use timer::Timer;

extern crate embedded_hal;

type BypassSwitch = Switch<PB0<Input<PullUp>>, PB3<Output>, PB2<Output>>;
static BYPASS_SWITCH: Mutex<RefCell<Option<BypassSwitch>>> = Mutex::new(RefCell::new(None));

type PresetSwitch = Switch<PB1<Input<PullUp>>, PB4<Output>, PB5<Output>>;
static PRESET_SWITCH: Mutex<RefCell<Option<PresetSwitch>>> = Mutex::new(RefCell::new(None));

static BYPASS_TIMER: Mutex<RefCell<Option<Timer>>> = Mutex::new(RefCell::new(None));
static PRESET_TIMER: Mutex<RefCell<Option<Timer>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let peripherals = attiny85_hal::pac::Peripherals::take().unwrap();

    let mut tc0 = peripherals.TC0;
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.ocr0a.write(|w| unsafe { w.bits(250 as u8) });
    tc0.tccr0b.write(|w| w.cs0.prescale_1024());

    let mut tc1 = peripherals.TC1;
    tc1.tccr1.write(|w| w.wgm1().ctc());
    tc1.ocr1a.write(|w| unsafe { w.bits(250 as u8) });
    tc1.tccr1.write(|w| w.cs0.prescale_1024());

    // peripherals.EXINT.gimsk.write(|w| w.pcie().set_bit());
    // peripherals.EXINT.pcmsk.write(|w| unsafe { w.bits(0b00000001) });

    let bypass_timer = Timer::new();
    let preset_timer = Timer::new();

    let mut portb = peripherals.PORTB.split();

    let bypass_input = portb.pb0.into_pull_up_input(&mut portb.ddr);
    let bypass_output = portb.pb3.into_output(&mut portb.ddr);
    let bypass_led = portb.pb2.into_output(&mut portb.ddr);
    let bypass = Switch::new(bypass_input, bypass_output, bypass_led, switch::Kind::BYPASS);

    let preset_input = portb.pb1.into_pull_up_input(&mut portb.ddr);
    let preset_output = portb.pb4.into_output(&mut portb.ddr);
    let preset_led = portb.pb5.into_output(&mut portb.ddr);
    let preset = Switch::new(preset_input, preset_output, preset_led, switch::Kind::PRESET);

    free(|cs| {
        BYPASS_SWITCH.borrow(cs).replace(Some(bypass));
        PRESET_SWITCH.borrow(cs).replace(Some(preset));
        BYPASS_TIMER.borrow(cs).replace(Some(bypass_timer));
        PRESET_TIMER.borrow(cs).replace(Some(preset_timer));
    });

    loop {
        free(|cs| {
            let mut bypass_ref = BYPASS_SWITCH.borrow(cs).borrow_mut();
            let bypass = bypass_ref.as_mut().unwrap();
            bypass.check();

            let mut preset_ref = PRESET_SWITCH.borrow(cs).borrow_mut();
            let preset = preset_ref.as_mut().unwrap();
            preset.check();
        })
    }
}

#[interrupt(attiny85)]
fn TIMER0_COMPA() {
    free(|cs| {
        let mut timer_ref = BYPASS_TIMER.borrow(cs).borrow_mut();
        let timer = timer_ref.as_mut().unwrap();
        timer.tick();
    })
}

#[interrupt(attiny85)]
fn TIMER1_COMPA() {
    free(|cs| {
        let mut timer_ref = PRESET_TIMER.borrow(cs).borrow_mut();
        let timer = timer_ref.as_mut().unwrap();
        timer.tick();
    })
}
