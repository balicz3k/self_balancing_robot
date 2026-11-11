use super::peripherals_mapping::MotorPeripherals;
use esp_idf_hal::ledc::{LedcDriver};

pub struct Motor<'d> {
    max_motor_speed: f32,
    forward_pwm_driver: LedcDriver<'d>,
    backward_pwm_driver: LedcDriver<'d>,
}

impl<'d> Motor<'d> {
    pub fn new(motor_peripherals: MotorPeripherals) -> Self {
        let mut motor = Self {
            max_motor_speed: motor_peripherals.forward_pwm_driver.get_max_duty() as f32,
            forward_pwm_driver: motor_peripherals.forward_pwm_driver,
            backward_pwm_driver: motor_peripherals.backward_pwm_driver,
        };
        motor.stop();
        motor
    }

    pub fn run_forward(&mut self, max_speed_percentage: f32) {
        let max_speed_percentage = max_speed_percentage.clamp(0.0, 100.0);
        let speed = (self.max_motor_speed * (max_speed_percentage / 100.0)) as u32;
        self.backward_pwm_driver.set_duty(0).unwrap();
        self.forward_pwm_driver.set_duty(speed).unwrap();
    }

    pub fn run_backward(&mut self, max_speed_percentage: f32) {
        let max_speed_percentage = max_speed_percentage.clamp(0.0, 100.0);
        let speed = (self.max_motor_speed * (max_speed_percentage / 100.0)) as u32;
        self.forward_pwm_driver.set_duty(0).unwrap();
        self.backward_pwm_driver.set_duty(speed).unwrap();
    }

    pub fn stop(&mut self) {
        self.forward_pwm_driver.set_duty(0).unwrap();
        self.backward_pwm_driver.set_duty(0).unwrap();
    }
}
