use esp_idf_hal::gpio::{AnyIOPin, AnyOutputPin};
use esp_idf_hal::i2c::I2C0;
use esp_idf_hal::ledc::{CHANNEL0, CHANNEL1, CHANNEL2, CHANNEL3, TIMER0};
use esp_idf_hal::peripherals::Peripherals;

pub struct MotorResources<C1, C2> {
    pub forward_pin: AnyOutputPin<'static>,
    pub forward_channel: C1,
    pub backward_pin: AnyOutputPin<'static>,
    pub backward_channel: C2,
}

pub struct DriveResources {
    pub left_motor_resources: MotorResources<CHANNEL0<'static>, CHANNEL1<'static>>,
    pub right_motor_resources: MotorResources<CHANNEL2<'static>, CHANNEL3<'static>>,
    pub pwm_timer: TIMER0<'static>,
    pub enable_motors_pin: AnyOutputPin<'static>,
}

pub struct I2cBusResources {
    pub sda_pin: AnyIOPin<'static>,
    pub scl_pin: AnyIOPin<'static>,
    pub i2c_controller: I2C0<'static>,
}

pub struct BoardResources {
    pub drive_resources: DriveResources,
    pub i2c_bus_resources: I2cBusResources,
}

impl BoardResources {
    pub fn new(peripherals: Peripherals) -> Self {
        Self {
            drive_resources: DriveResources {
                left_motor_resources: MotorResources {
                    forward_pin: peripherals.pins.gpio4.into(),
                    forward_channel: peripherals.ledc.channel0.into(),
                    backward_pin: peripherals.pins.gpio5.into(),
                    backward_channel: peripherals.ledc.channel1.into(),
                },
                right_motor_resources: MotorResources {
                    forward_pin: peripherals.pins.gpio6.into(),
                    forward_channel: peripherals.ledc.channel2.into(),
                    backward_pin: peripherals.pins.gpio7.into(),
                    backward_channel: peripherals.ledc.channel3.into(),
                },
                pwm_timer: peripherals.ledc.timer0,
                enable_motors_pin: peripherals.pins.gpio15.into(),
            },
            i2c_bus_resources: I2cBusResources {
                sda_pin: peripherals.pins.gpio9.into(),
                scl_pin: peripherals.pins.gpio8.into(),
                i2c_controller: peripherals.i2c0,
            },
        }
    }
}
