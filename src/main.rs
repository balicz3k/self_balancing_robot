mod components;

use esp_idf_hal::delay::{FreeRtos, TickType};
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::i2c::*;
use esp_idf_hal::peripherals::Peripherals;

use components::motor::Motor;
use components::peripherals_mapping::PeripheralsMapping;

const I2C_TIMEOUT: u32 = TickType::new_millis(50).ticks();

fn scan_adresses(i2c: &mut I2cDriver) {
    for adres in 0x00u8..=0x7Fu8 {
        let res = i2c.write(adres, &[], 3);
        if res.is_ok() {
            log::info!("address: {:#X} works", adres);
        }
    }
}

fn wake_up_gyroscope(i2c: &mut I2cDriver) {
    const DEVICE_ADRESS: u8 = 0x68;
    const BYTES: &[u8] = &[0x6B, 0x00];

    let res = i2c.write(DEVICE_ADRESS, BYTES, I2C_TIMEOUT);
    if res.is_err() {
        log::error!("Error writing to device: {:?}", res.err());
        return;
    }
}

fn split_movement_params(buffer: &[u8; 14]) -> (i16, i16, i16, i16, i16, i16) {
    let ax = ((buffer[0] as i16) << 8) | (buffer[1] as i16);
    let ay = ((buffer[2] as i16) << 8) | (buffer[3] as i16);
    let az = ((buffer[4] as i16) << 8) | (buffer[5] as i16);
    let gx = ((buffer[8] as i16) << 8) | (buffer[9] as i16);
    let gy = ((buffer[10] as i16) << 8) | (buffer[11] as i16);
    let gz = ((buffer[12] as i16) << 8) | (buffer[13] as i16);

    (ax, ay, az, gx, gy, gz)
}

fn read_movement(i2c: &mut I2cDriver) {
    const DEVICE_ADRESS: u8 = 0x68;
    const DEVICE_REGISTER: &[u8] = &[0x3B];

    let mut buffer = [0u8; 14];
    let res = i2c.write_read(DEVICE_ADRESS, DEVICE_REGISTER, &mut buffer, I2C_TIMEOUT);
    if res.is_err() {
        log::error!("Error reading from device: {:?}", res.err());
        return;
    }
    let params = split_movement_params(&buffer);
    log::info!(
        "ax: {}, ay: {}, az: {}\ngx: {}, gy: {}, gz: {}",
        params.0,
        params.1,
        params.2,
        params.3,
        params.4,
        params.5
    );
}

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let pheripherals = Peripherals::take().unwrap();
    let pin_mapping = PeripheralsMapping::new(pheripherals);

    let config = I2cConfig::new().baudrate(esp_idf_hal::units::Hertz(400000));
    let mut i2c: I2cDriver = I2cDriver::new(
        pin_mapping.imu_pins.i2c,
        pin_mapping.imu_pins.sda_pin,
        pin_mapping.imu_pins.scl_pin,
        &config,
    )
    .unwrap();
    scan_adresses(&mut i2c);

    wake_up_gyroscope(&mut i2c);

    let mut eep = PinDriver::output(pin_mapping.drive_pins.enable_motor_pin).unwrap();
    eep.set_high().unwrap();

    let mut left_motor = Motor::new(pin_mapping.drive_pins.left_motor_pins);
    let mut right_motor = Motor::new(pin_mapping.drive_pins.right_motor_pins);

    loop {
        read_movement(&mut i2c);
        right_motor.brake();
        left_motor.run_forward(0);
        log::info!("left motor run forward");
        FreeRtos::delay_ms(10000);
        left_motor.stop();
        right_motor.run_forward(0);
        log::info!("right motor run forward");
        FreeRtos::delay_ms(10000);
    }
}
