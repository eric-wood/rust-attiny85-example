extern crate attiny85_hal as hal;
extern crate embedded_hal;

// Using multiples of 10 so as not to overflow
static HOLD_TIME_TEN_MS: u8 = 70;

pub struct Timer {
    ticks: u8,
    pub threshold_reached: bool,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            ticks: 0,
            threshold_reached: false,
        }
    }

    pub fn reset(&mut self) {
        self.threshold_reached = false;
        self.ticks = 0;
    }

    pub fn tick(&mut self) {
        if self.threshold_reached {
            return;
        }

        self.ticks += 1;

        if self.ticks >= HOLD_TIME_TEN_MS {
            self.threshold_reached = true
        }
    }
}
