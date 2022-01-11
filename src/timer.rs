extern crate attiny85_hal as hal;
extern crate embedded_hal;

pub struct Timer {
    ticks_ms: u8,
    ticks: u8,
    threshold: u8,
    threshold_division: u8,
    pub threshold_reached: bool,
}

impl Timer {
    pub fn new(threshold: u8, threshold_division: u8) -> Self {
        Timer {
            ticks_ms: 0,
            ticks: 0,
            threshold,
            threshold_division,
            threshold_reached: false,
        }
    }

    pub fn reset(&mut self) {
        self.threshold_reached = false;
        self.ticks = 0;
        self.ticks_ms = 0;
    }

    pub fn tick(&mut self) {
        if self.threshold_reached {
            return;
        }

        self.ticks_ms += 1;

        if self.ticks_ms >= self.threshold_division {
            self.ticks += 1;
            self.ticks_ms = 0;

            if self.ticks >= self.threshold {
                self.threshold_reached = true
            }
        }
    }
}
