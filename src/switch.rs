extern crate attiny85_hal;
extern crate embedded_hal as hal;
use crate::SwitchTimer;
use core::fmt::Debug;
use embedded_hal::digital::v2::{InputPin, OutputPin};

pub struct Switch<Input, Output> {
    input: Input,
    output: Output,
    active: bool,
    previous_state: bool,
}

impl<Input, Output> Switch<Input, Output>
where
    Input: InputPin,
    Output: OutputPin,
    Input::Error: Debug,
    Output::Error: Debug,
{
    pub fn new(input: Input, output: Output) -> Self {
        Switch {
            input,
            output,
            active: false,
            previous_state: false,
        }
    }

    pub fn on_change(&mut self, timer: &mut SwitchTimer) {
        let pressed = self.is_pressed();

        if pressed == self.previous_state || !timer.debounce.threshold_reached {
            return;
        }

        timer.debounce.reset();

        self.previous_state = pressed;

        if pressed {
            timer.hold.reset();

            self.set_state(!self.active);
        } else {
            if timer.hold.threshold_reached {
                self.set_state(false);
            }
        }
    }

    fn is_pressed(&mut self) -> bool {
        self.input.is_low().unwrap()
    }

    fn set_state(&mut self, state: bool) {
        self.active = state;
        self.set_switch(state);
    }

    fn set_switch(&mut self, state: bool) {
        if state {
            self.output.set_high().unwrap();
        } else {
            self.output.set_low().unwrap();
        }
    }
}
