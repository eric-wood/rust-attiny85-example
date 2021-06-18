extern crate attiny85_hal;
extern crate embedded_hal as hal;
use crate::Timer;
use avr_device::interrupt::{free, Mutex};
use cell::RefCell;
use core::cell;
use core::fmt::Debug;
use embedded_hal::digital::v2::{InputPin, OutputPin};

type TimerMutex = &'static Mutex<RefCell<Option<Timer>>>;

pub struct Switch<Input, Output, Led> {
    input: Input,
    output: Output,
    led: Led,
    active: bool,
    was_pressed: bool,
    timer: TimerMutex,
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
    pub fn new(input: Input, output: Output, led: Led, timer: TimerMutex) -> Self {
        Switch {
            input,
            output,
            led,
            timer,
            active: false,
            was_pressed: false,
        }
    }

    pub fn on_change(&mut self) {
        let pressed = self.is_pressed();

        if !self.was_pressed && pressed {
            self.was_pressed = true;

            free(|cs| {
                let mut timer_ref = self.timer.borrow(cs).borrow_mut();
                let timer = timer_ref.as_mut().unwrap();
                timer.start();
            });

            self.set_state(!self.active);
        } else if self.was_pressed && !pressed {
            self.was_pressed = false;

            self.handle_momentary();
        }
    }

    fn is_pressed(&mut self) -> bool {
        self.input.is_low().unwrap()
    }

    fn set_state(&mut self, state: bool) {
        self.active = state;
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

    fn handle_momentary(&mut self) {
        free(|cs| {
            let mut timer_ref = self.timer.borrow(cs).borrow_mut();
            let timer = timer_ref.as_mut().unwrap();
            if timer.threshold_reached {
                self.set_state(false);
                timer.stop();
            }
        });
    }
}
