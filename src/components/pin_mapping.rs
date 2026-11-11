use esp_idf_hal::gpio::{AnyIOPin, AnyOutputPin};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::i2c::I2C0;

pub struct MotorPins {
    pub forward_pin: AnyOutputPin<'static>,
    pub backward_pin: AnyOutputPin<'static>,
}

pub struct DrivePins {
    pub left_motor_pins: MotorPins,
    pub right_motor_pins: MotorPins,
    pub enable_motor_pin: AnyOutputPin<'static>,
}

pub struct ImuPins {
    pub sda_pin: AnyIOPin<'static>,
    pub scl_pin: AnyIOPin<'static>,
    pub i2c: I2C0<'static>
}

pub struct PinMapping {
    pub drive_pins: DrivePins,
    pub imu_pins: ImuPins,
}

impl PinMapping {
    pub fn new(pheriperals: Peripherals) -> Self {
        Self {
            drive_pins: DrivePins {
                left_motor_pins: MotorPins {
                    forward_pin: pheriperals.pins.gpio4.into(),
                    backward_pin: pheriperals.pins.gpio5.into(),
                },
                right_motor_pins: MotorPins {
                    forward_pin: pheriperals.pins.gpio6.into(),
                    backward_pin: pheriperals.pins.gpio7.into(),
                },
                enable_motor_pin: pheriperals.pins.gpio15.into(),
            },
            imu_pins: ImuPins {
                sda_pin: pheriperals.pins.gpio9.into(),
                scl_pin: pheriperals.pins.gpio8.into(),
                i2c: pheriperals.i2c0,
            }
        }
    }
}
