use esp_idf_hal::gpio::{AnyIOPin, AnyOutputPin};
use esp_idf_hal::i2c::I2C0;
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::ledc::{LedcDriver, LedcTimerDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::Hertz;

pub struct MotorPeripherals {
    pub forward_pwm_driver: LedcDriver<'static>,
    pub backward_pwm_driver: LedcDriver<'static>,
}

pub struct DrivePeripherals {
    pub left_motor_pins: MotorPeripherals,
    pub right_motor_pins: MotorPeripherals,
    pub enable_motor_pin: AnyOutputPin<'static>,
}

pub struct ImuI2cBus {
    pub sda_pin: AnyIOPin<'static>,
    pub scl_pin: AnyIOPin<'static>,
    pub i2c: I2C0<'static>,
}

pub struct PeripheralsMapping {
    pub drive_pins: DrivePeripherals,
    pub imu_pins: ImuI2cBus,
}

impl PeripheralsMapping {
    pub fn new(peripherals: Peripherals) -> Self {
        let timer_config = TimerConfig::new().frequency(Hertz(20_000));
        let pwm_timer = LedcTimerDriver::new(peripherals.ledc.timer0, &timer_config).unwrap();
        Self {
            drive_pins: DrivePeripherals {
                left_motor_pins: MotorPeripherals {
                    forward_pwm_driver: LedcDriver::new(
                        peripherals.ledc.channel0,
                        &pwm_timer,
                        peripherals.pins.gpio4,
                    )
                    .unwrap(),
                    backward_pwm_driver: LedcDriver::new(
                        peripherals.ledc.channel1,
                        &pwm_timer,
                        peripherals.pins.gpio5,
                    )
                    .unwrap(),
                },
                right_motor_pins: MotorPeripherals {
                    forward_pwm_driver: LedcDriver::new(
                        peripherals.ledc.channel2,
                        &pwm_timer,
                        peripherals.pins.gpio6,
                    )
                    .unwrap(),
                    backward_pwm_driver: LedcDriver::new(
                        peripherals.ledc.channel3,
                        &pwm_timer,
                        peripherals.pins.gpio7,
                    )
                    .unwrap(),
                },
                enable_motor_pin: peripherals.pins.gpio15.into(),
            },
            imu_pins: ImuI2cBus {
                sda_pin: peripherals.pins.gpio9.into(),
                scl_pin: peripherals.pins.gpio8.into(),
                i2c: peripherals.i2c0,
            },
        }
    }
}
