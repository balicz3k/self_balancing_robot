mod components;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::units::Hertz;

use esp_idf_hal::ledc::{config::TimerConfig, LedcTimerDriver};

use components::board_resources::BoardResources;
use components::imu::Imu;
use components::motor::Motor;

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

    let mut eep = PinDriver::output(board_resources.drive_resources.enable_motors_pin).unwrap();
    eep.set_high().unwrap();

    let timer_config = TimerConfig::new().frequency(Hertz(20_000));
    let pwm_timer =
        LedcTimerDriver::new(board_resources.drive_resources.pwm_timer, &timer_config).unwrap();
    let mut left_motor = Motor::new(
        board_resources.drive_resources.left_motor_resources,
        &pwm_timer,
    );
    let mut right_motor = Motor::new(
        board_resources.drive_resources.right_motor_resources,
        &pwm_timer,
    );

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
