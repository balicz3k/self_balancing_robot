use super::board_resources::MotorResources;
use esp_idf_hal::ledc::{LedcChannel, LedcDriver, LedcTimerDriver, LowSpeed};

pub struct Motor {
    max_motor_speed: f32,
    forward_pwm_driver: LedcDriver<'static>,
    backward_pwm_driver: LedcDriver<'static>,
}

impl Motor {
    pub fn new<
        C1: LedcChannel<SpeedMode = LowSpeed> + 'static,
        C2: LedcChannel<SpeedMode = LowSpeed> + 'static,
    >(
        motor_resources: MotorResources<C1, C2>,
        timer: &LedcTimerDriver<'static, LowSpeed>,
    ) -> Self {
        let motor_forward_driver = LedcDriver::new(
            motor_resources.forward_channel,
            timer,
            motor_resources.forward_pin,
        )
        .unwrap();
        let motor_backward_driver = LedcDriver::new(
            motor_resources.backward_channel,
            timer,
            motor_resources.backward_pin,
        )
        .unwrap();
    
        let mut motor = Self {
            max_motor_speed: motor_forward_driver.get_max_duty() as f32,
            forward_pwm_driver: motor_forward_driver,
            backward_pwm_driver: motor_backward_driver,
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
