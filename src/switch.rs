extern crate attiny85_hal;
extern crate embedded_hal as hal;
use embedded_hal::digital::v2::{InputPin, OutputPin};
use core::fmt::Debug;

pub struct Switch<Input, Output, Led> {
    input: Input,
    output: Output,
    led: Led,
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
    pub fn new(input: Input, output: Output, led: Led) -> Self {
        Switch {
            input,
            output,
            led,
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

            // TODO: check to see if minimum time limit has been met
            // if it has been, turn the effect off
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
}
