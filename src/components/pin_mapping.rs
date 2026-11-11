use esp_idf_hal::gpio::{AnyIOPin, AnyOutputPin};
use esp_idf_hal::peripherals::Peripherals;

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
}

pub struct PinMapping {
    pub drive_pins: DrivePins,
    pub imu_pins: ImuPins,
}

impl PinMapping {
    pub fn new() -> Self {
        let pheriperals_pins = Peripherals::take().unwrap().pins;

        Self {
            drive_pins: DrivePins {
                left_motor_pins: MotorPins {
                    forward_pin: pheriperals_pins.gpio4.into(),
                    backward_pin: pheriperals_pins.gpio5.into(),
                },
                right_motor_pins: MotorPins {
                    forward_pin: pheriperals_pins.gpio6.into(),
                    backward_pin: pheriperals_pins.gpio7.into(),
                },
                enable_motor_pin: pheriperals_pins.gpio15.into(),
            },
            imu_pins: ImuPins {
                sda_pin: pheriperals_pins.gpio9.into(),
                scl_pin: pheriperals_pins.gpio8.into(),
            },
        }
    }
}
