#![feature(llvm_asm)]
#![no_main]
#![no_std]
#![feature(abi_avr_interrupt)]

extern crate panic_halt;

extern crate avr_device;
use avr_device::interrupt;
use avr_device::interrupt::{free, Mutex};

extern crate attiny85_hal as hal;

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

type InterruptFlag = Mutex<RefCell<bool>>;
static TIMER_INTERRUPT: InterruptFlag = Mutex::new(RefCell::new(false));
static BUTTON_INTERRUPT: InterruptFlag = Mutex::new(RefCell::new(false));

static DEBOUNCE_TIME_MS: u8 = 7;
// Note that this is scaled by 10 so as not to overflow!
static HOLD_TIME_TEN_MS: u8 = 70;

#[entry]
fn main() -> ! {
    let peripherals = attiny85_hal::pac::Peripherals::take().unwrap();

    // Configure timer/counter 0 to count up and fire the TIMER0_COMPA
    // at a regular interval to act as a clock for our timers
    // The compare interrupt is set to fire roughly every 1ms:
    // 1 / (1Mhz / 8) * 125 = 1ms
    let tc0 = peripherals.TC0;
    tc0.tccr0a.write(|w| w.wgm0().ctc());
    tc0.tccr0b.write(|w| w.cs0().prescale_8());
    tc0.ocr0a.write(|w| unsafe { w.bits(124 as u8) });
    tc0.timsk.write(|w| w.ocie0a().bit(true));

    // Enable pin change interrupt for PB0 and PB1 to detect switch changes
    peripherals.EXINT.gimsk.write(|w| w.pcie().set_bit());
    peripherals
        .EXINT
        .pcmsk
        .write(|w| unsafe { w.bits(0b00011000) });

    let mut bypass_debounce_timer = Timer::new(DEBOUNCE_TIME_MS, 0);
    let mut bypass_hold_timer = Timer::new(HOLD_TIME_TEN_MS, 10);

    let mut portb = peripherals.PORTB.split();

    let bypass_input = portb.pb3.into_pull_up_input(&mut portb.ddr);
    let bypass_output = portb.pb0.into_output(&mut portb.ddr);
    let mut bypass = Switch::new(bypass_input, bypass_output);

    unsafe { avr_device::interrupt::enable() };

    loop {
        avr_device::asm::sleep();

        let (timer, button) = free(|cs| {
            (
                TIMER_INTERRUPT.borrow(cs).replace(false),
                BUTTON_INTERRUPT.borrow(cs).replace(false),
            )
        });

        if timer {
            bypass_debounce_timer.tick();
            bypass_hold_timer.tick();
        }
        if button {
            bypass.on_change(&mut bypass_debounce_timer, &mut bypass_hold_timer);
        }
    }
}

#[interrupt(attiny85)]
fn TIMER0_COMPA() {
    free(|cs| {
        TIMER_INTERRUPT.borrow(cs).replace(true);
    })
}

#[interrupt(attiny85)]
fn PCINT0() {
    free(|cs| {
        BUTTON_INTERRUPT.borrow(cs).replace(true);
    })
}
