mod components;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use components::board_resources::BoardResources;
use components::imu::Imu;
use components::drive_train::DriveTrain;

fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let board_resources = BoardResources::new(peripherals);

    let mut imu = Imu::new(board_resources.i2c_bus_resources);

    imu.activate();

    let mut drive_train = DriveTrain::new(board_resources.drive_resources);


    let mut speed: f32 = 50.0 as f32;
    loop {
        println!("{:?}", imu.get_data());
        drive_train.run_forward(speed);
        log::info!("both motors run forward with speed: {}", speed);
        FreeRtos::delay_ms(1000);
        speed += 1.0;
    }
}
