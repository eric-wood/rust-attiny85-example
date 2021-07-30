extern crate attiny85_hal;
extern crate embedded_hal as hal;
use crate::Timer;
use avr_device::interrupt::{free, Mutex};
use cell::RefCell;
use core::cell;
use core::fmt::Debug;
use embedded_hal::digital::v2::{InputPin, OutputPin};

type TimerMutex = &'static Mutex<RefCell<Option<Timer>>>;

pub struct Switch<Input, Output> {
    input: Input,
    output: Output,
    active: bool,
    previous_state: bool,
    timer: TimerMutex,
}

impl<Input, Output> Switch<Input, Output>
where
    Input: InputPin,
    Output: OutputPin,
    Input::Error: Debug,
    Output::Error: Debug,
{
    pub fn new(input: Input, output: Output, timer: TimerMutex) -> Self {
        Switch {
            input,
            output,
            timer,
            active: false,
            previous_state: false,
        }
    }

    pub fn on_change(&mut self) {
        let pressed = self.is_pressed();

        if pressed == self.previous_state {
            return;
        }

        self.previous_state = pressed;

        if pressed {
            free(|cs| {
                let mut timer_ref = self.timer.borrow(cs).borrow_mut();
                let timer = timer_ref.as_mut().unwrap();
                timer.reset();
            });

            self.set_state(!self.active);
        } else  {
            self.handle_momentary();
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

    fn handle_momentary(&mut self) {
        free(|cs| {
            let mut timer_ref = self.timer.borrow(cs).borrow_mut();
            let timer = timer_ref.as_mut().unwrap();
            if timer.threshold_reached {
                self.set_state(false);
            }
        });
    }
}
