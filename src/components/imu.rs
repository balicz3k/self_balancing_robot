use super::board_resources::I2cBusResources;
use esp_idf_hal::{
    delay::TickType,
    i2c::{I2cConfig, I2cDriver},
    units::Hertz,
};

const DEVICE_ADDRESS: u8 = 0x68;
const I2C_TIMEOUT: u32 = TickType::new_millis(50).ticks();
const BAUDRATE: Hertz = Hertz(400_000);
const REG_PWR_MGMT_1: u8 = 0x6B;
const PWR_MGMT_1_CLK_PLL_X: u8 = 0x01;
const REG_ACCEL_XOUT_H: u8 = 0x3B;
pub const IMU_DATA_LEN: usize = 14;

#[derive(Debug)]
pub struct ImuData {
    pub ax: i16,
    pub ay: i16,
    pub az: i16,
    pub gx: i16,
    pub gy: i16,
    pub gz: i16,
}

impl From<[u8; IMU_DATA_LEN]> for ImuData {
    fn from(b: [u8; IMU_DATA_LEN]) -> Self {
        let word = |i: usize| i16::from_be_bytes([b[i], b[i + 1]]);
        Self {
            ax: word(0),
            ay: word(2),
            az: word(4),
            gx: word(8),
            gy: word(10),
            gz: word(12),
        }
    }
}

pub struct Imu<'d> {
    i2c_driver: I2cDriver<'d>,
}

impl<'d> Imu<'d> {
    pub fn new(i2c_bus_resources: I2cBusResources) -> Self {
        let i2c_config = I2cConfig::new().baudrate(BAUDRATE);
        let i2c_driver = I2cDriver::new(
            i2c_bus_resources.i2c_controller,
            i2c_bus_resources.sda_pin,
            i2c_bus_resources.scl_pin,
            &i2c_config,
        )
        .unwrap();
        Self { i2c_driver }
    }

    pub fn activate(&mut self) {
        self.i2c_driver
            .write(
                DEVICE_ADDRESS,
                &[REG_PWR_MGMT_1, PWR_MGMT_1_CLK_PLL_X],
                I2C_TIMEOUT,
            )
            .unwrap();
    }

    pub fn get_data(&mut self) -> ImuData {
        let mut buffer = [0u8; IMU_DATA_LEN];
        self.i2c_driver
            .write_read(
                DEVICE_ADDRESS,
                &[REG_ACCEL_XOUT_H],
                &mut buffer,
                I2C_TIMEOUT,
            )
            .unwrap();
        buffer.into()
    }
}
