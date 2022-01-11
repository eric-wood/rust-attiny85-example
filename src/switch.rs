extern crate attiny85_hal;
extern crate embedded_hal as hal;
use crate::TimerMutex;
use avr_device::interrupt::free;
use core::fmt::Debug;
use embedded_hal::digital::v2::{InputPin, OutputPin};

pub struct Switch<Input, Output> {
    input: Input,
    output: Output,
    active: bool,
    previous_state: bool,
    debounce_timer: &'static TimerMutex,
    hold_timer: &'static TimerMutex,
}

impl<Input, Output> Switch<Input, Output>
where
    Input: InputPin,
    Output: OutputPin,
    Input::Error: Debug,
    Output::Error: Debug,
{
    pub fn new(
        input: Input,
        output: Output,
        debounce_timer: &'static TimerMutex,
        hold_timer: &'static TimerMutex,
    ) -> Self {
        Switch {
            input,
            output,
            debounce_timer,
            hold_timer,
            active: false,
            previous_state: false,
        }
    }

    pub fn on_change(&mut self) {
        free(|cs| {
            let mut debounce_timer_ref = self.debounce_timer.borrow(cs).borrow_mut();
            let debounce_timer = debounce_timer_ref.as_mut().unwrap();
            let mut hold_timer_ref = self.hold_timer.borrow(cs).borrow_mut();
            let hold_timer = hold_timer_ref.as_mut().unwrap();

            let pressed = self.is_pressed();

            if pressed == self.previous_state || !debounce_timer.threshold_reached {
                return;
            }

            debounce_timer.reset();

            self.previous_state = pressed;

            if pressed {
                hold_timer.reset();

                self.set_state(!self.active);
            } else {
                if hold_timer.threshold_reached {
                    self.set_state(false);
                }
            }
        });
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
