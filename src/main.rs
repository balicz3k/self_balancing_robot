use esp_idf_hal::gpio::{Output, PinDriver};
use esp_idf_hal::i2c::*;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::delay::{FreeRtos, TickType};

const I2C_TIMEOUT: u32 = TickType::new_millis(50).ticks();

fn scan_adresses(i2c: &mut I2cDriver) {
    for adres in 0x00u8..=0x7Fu8
    {
        let res = i2c.write(adres, &[], 3);
        if res.is_ok()
        {
            log::info!("address: {:#X} works", adres);
        }

    }
}

fn wake_up_gyroscope(i2c: &mut I2cDriver) {
    const DEVICE_ADRESS: u8 = 0x68;
    const BYTES: &[u8] = &[0x6B, 0x00];

    let res = i2c.write(DEVICE_ADRESS, BYTES, I2C_TIMEOUT);
    if res.is_err()
    {
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
    if res.is_err()
    {
        log::error!("Error reading from device: {:?}", res.err());
        return;
    }
    let params = split_movement_params(&buffer);
    log::info!("ax: {}, ay: {}, az: {}\ngx: {}, gy: {}, gz: {}", params.0, params.1, params.2, params.3, params.4, params.5);
}


fn change_direction(in_a: &mut PinDriver<'_, Output>, in_b: &mut PinDriver<'_, Output>) {
    if in_a.is_set_high(){
        in_a.set_low().unwrap();
        in_b.set_high().unwrap();
    } else {
        in_a.set_high().unwrap();
        in_b.set_low().unwrap();
    }
}

fn start_engine(in_a: &mut PinDriver<'_, Output>, in_b: &mut PinDriver<'_, Output>) {
    in_a.set_high().unwrap();
    in_b.set_low().unwrap();
}

fn stop_engine(in_a: &mut PinDriver<'_, Output>, in_b: &mut PinDriver<'_, Output>) {
    in_a.set_low().unwrap();
    in_b.set_low().unwrap();
}

fn hard_stop_engine(in_a: &mut PinDriver<'_, Output>, in_b: &mut PinDriver<'_, Output>) {
    in_a.set_high().unwrap();
    in_b.set_high().unwrap();
}


fn main() {
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let pheripherals = Peripherals::take().unwrap();
    let config = I2cConfig::new().baudrate(esp_idf_hal::units::Hertz(400000));
    let scl = pheripherals.pins.gpio8;
    let sda = pheripherals.pins.gpio9;
    let mut i2c: I2cDriver = I2cDriver::new(pheripherals.i2c0, sda, scl, &config).unwrap();
    scan_adresses(&mut i2c);

    wake_up_gyroscope(&mut i2c);

    let mut eep = PinDriver::output(pheripherals.pins.gpio15).unwrap();
    eep.set_high().unwrap();

    let mut engine1_left = PinDriver::output(pheripherals.pins.gpio4).unwrap();
    let mut engine1_right = PinDriver::output(pheripherals.pins.gpio5).unwrap();

    engine1_left.set_high().unwrap();
    engine1_right.set_low().unwrap();

    let mut engine2_left = PinDriver::output(pheripherals.pins.gpio6).unwrap();
    let mut engine2_right = PinDriver::output(pheripherals.pins.gpio7).unwrap();

    engine2_left.set_high().unwrap();
    engine2_right.set_low().unwrap();
    
    loop{
        eep.set_high().unwrap();
        FreeRtos::delay_ms(1000);
        read_movement(&mut i2c);    
        change_direction(&mut engine1_left, &mut engine1_right);

        FreeRtos::delay_ms(10000);
        hard_stop_engine(&mut engine1_left, &mut engine1_right);
        hard_stop_engine(&mut engine2_left, &mut engine2_right);
        FreeRtos::delay_ms(10000);

        start_engine(&mut engine1_left, &mut engine1_right);
        start_engine(&mut engine2_left, &mut engine2_right);

        FreeRtos::delay_ms(10000);

        change_direction(&mut engine2_left, &mut engine2_right);
    }
}
