extern crate attiny85_hal;
extern crate embedded_hal as hal;
use core::fmt::Debug;
use avr_device::interrupt::free;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use crate::{BYPASS_TIMER, PRESET_TIMER};

pub enum Kind {
    BYPASS,
    PRESET,
}

static HOLD_TIME_MS: u8 = 100;

pub struct Switch<Input, Output, Led> {
    input: Input,
    output: Output,
    led: Led,
    kind: Kind,
    active: bool,
    was_pressed: bool,
}

impl<Input, Output, Led> Switch<Input, Output, Led>
where
    Input: InputPin,
    Output: OutputPin,
    Led: OutputPin,
    Input::Error: Debug,
    Output::Error: Debug,
    Led::Error: Debug,
{
    pub fn new(input: Input, output: Output, led: Led, kind: Kind) -> Self {
        Switch {
            input,
            output,
            led,
            kind,
            active: false,
            was_pressed: false,
        }
    }

    pub fn check(&mut self) {
        let pressed = self.is_pressed();

        if !self.was_pressed && pressed {
            self.active = !self.active;
            self.was_pressed = true;

            self.set_state(self.active);
        } else if self.was_pressed && !pressed {
            self.was_pressed = false;

            self.handle_momentary();
        }
    }

    fn is_pressed(&mut self) -> bool {
        self.input.is_low().unwrap()
    }

    fn set_state(&mut self, state: bool) {
        self.set_led(state);
        self.set_switch(state);
    }

    fn set_led(&mut self, state: bool) {
        if state {
            self.led.set_high().unwrap();
        } else {
            self.led.set_low().unwrap();
        }
    }

    fn set_switch(&mut self, state: bool) {
        if state {
            self.output.set_high().unwrap();
        } else {
            self.output.set_low().unwrap();
        }
    }

    fn handle_momentary(&self) {
        let elapsed: u8 = 0;

        free(|cs| {
            match self.kind {
                BYPASS => {
                    let mut timer_ref = BYPASS_TIMER.borrow(cs).borrow_mut();
                    let timer = timer_ref.as_mut().unwrap();
                    elapsed = timer.elapsed_time_ms();
                },
                PRESET => {
                    let mut timer_ref = PRESET_TIMER.borrow(cs).borrow_mut();
                    let timer = timer_ref.as_mut().unwrap();
                    elapsed = timer.elapsed_time_ms();
                }
            };
        });

        if elapsed >= HOLD_TIME_MS {
            self.active = false;
            self.set_state(self.active);
        }
    }
}
