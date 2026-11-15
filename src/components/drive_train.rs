use super::board_resources::DriveResources;
use super::motor::Motor;
use esp_idf_hal::gpio::{Output, PinDriver};
use esp_idf_hal::ledc::{config::TimerConfig, LedcTimerDriver, LowSpeed};
use esp_idf_hal::units::Hertz;

pub enum Direction {
    LEFT,
    RIGHT,
}
struct DriveTrain {
    left_motor: Motor,
    right_motor: Motor,
    enable_motor_driver: PinDriver<'static, Output>,
    timer: LedcTimerDriver<'static, LowSpeed>,
}

impl DriveTrain {
    pub fn new(drive_resources: DriveResources) -> Self {
        let timer_config = TimerConfig::new().frequency(Hertz(20_000));
        let pwm_timer = LedcTimerDriver::new(drive_resources.pwm_timer, &timer_config).unwrap();
        Self {
            left_motor: Motor::new(drive_resources.left_motor_resources, &pwm_timer),
            right_motor: Motor::new(drive_resources.right_motor_resources, &pwm_timer),
            enable_motor_driver: PinDriver::output(drive_resources.enable_motors_pin).unwrap(),
            timer: pwm_timer,
        }
    }

    pub fn run_forward(&mut self, max_speed_percentage: f32) {
        if self.enable_motor_driver.is_set_low() {
            self.enable_motor_driver.set_high().unwrap();
        }
        self.left_motor.run_forward(max_speed_percentage);
        self.right_motor.run_forward(max_speed_percentage);
    }

    pub fn run_backward(&mut self, max_speed_percentage: f32) {
        if self.enable_motor_driver.is_set_low() {
            self.enable_motor_driver.set_high().unwrap();
        }
        self.left_motor.run_backward(max_speed_percentage);
        self.right_motor.run_backward(max_speed_percentage);
    }

    pub fn stop(&mut self) {
        self.left_motor.stop();
        self.right_motor.stop();
    }

    pub fn turn(&mut self, _direction: Direction, _rotation: f32) {
        // to be implemented
    }
}
