use esp_idf_hal::gpio::{Output, PinDriver};

use super::pin_mapping::MotorPins;

pub struct Motor<'d> {
    forward_pin_driver: PinDriver<'d, Output>,
    backward_pin_driver: PinDriver<'d, Output>,
}

impl<'d> Motor<'d> {
    pub fn new(motor_pins: MotorPins) -> Self {
        let mut motor = Self {
            forward_pin_driver: PinDriver::output(motor_pins.forward_pin).unwrap(),
            backward_pin_driver: PinDriver::output(motor_pins.backward_pin).unwrap(),
        };
        motor.stop();
        motor
    }

    pub fn run_forward(&mut self, _speed: u8) {
        self.backward_pin_driver.set_low().unwrap();
        self.forward_pin_driver.set_high().unwrap();
    }

    pub fn _run_backward(&mut self, _speed: u8) {
        self.forward_pin_driver.set_low().unwrap();
        self.backward_pin_driver.set_high().unwrap();
    }

    pub fn brake(&mut self) {
        self.forward_pin_driver.set_high().unwrap();
        self.backward_pin_driver.set_high().unwrap();
    }

    pub fn stop(&mut self) {
        self.forward_pin_driver.set_low().unwrap();
        self.backward_pin_driver.set_low().unwrap();
    }
}
