set -ex

cargo build --release

avrdude -c usbtiny -p attiny85 -U flash:w:target/avr-attiny85/release/attiny85-test.elf