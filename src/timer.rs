extern crate attiny85_hal as hal;
extern crate embedded_hal;

// Using multiples of 10 so as not to overflow
static HOLD_TIME_TEN_MS: u8 = 50;

pub struct Timer {
    ticks: u8,
    stopped: bool,
    pub threshold_reached: bool,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            ticks: 0,
            stopped: true,
            threshold_reached: false,
        }
    }

    pub fn start(&mut self) {
        self.stopped = false;
    }

    pub fn stop(&mut self) {
        self.stopped = true;
        self.threshold_reached = false;
        self.ticks = 0;
    }

    pub fn tick(&mut self) {
        if self.stopped || self.threshold_reached {
            return;
        }

        self.ticks += 1;

        if self.ticks >= HOLD_TIME_TEN_MS {
            self.threshold_reached = true
        }
    }
}
