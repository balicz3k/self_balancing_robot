mod components;

use esp_idf_hal::delay::{FreeRtos, TickType};
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::i2c::*;
use esp_idf_hal::peripherals::Peripherals;

use components::imu::Imu;
use components::motor::Motor;
use components::peripherals_mapping::PeripheralsMapping;

const I2C_TIMEOUT: u32 = TickType::new_millis(50).ticks();

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let pin_mapping = PeripheralsMapping::new(peripherals);

    let mut imu = Imu::new(pin_mapping.imu_pins);

    imu.activate();

    let mut eep = PinDriver::output(pin_mapping.drive_pins.enable_motor_pin).unwrap();
    eep.set_high().unwrap();

    let mut left_motor = Motor::new(pin_mapping.drive_pins.left_motor_pins);
    let mut right_motor = Motor::new(pin_mapping.drive_pins.right_motor_pins);

    let mut speed = 20.0 as f32;

    loop {
        println!("{:?}", imu.get_data());
        left_motor.run_forward(speed);
        right_motor.run_forward(speed);
        log::info!("both motors run forward with speed: {}", speed);
        FreeRtos::delay_ms(1000);
        speed += 1.0;
    }
}
