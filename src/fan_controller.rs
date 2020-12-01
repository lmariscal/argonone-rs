use rppal::i2c::I2c;
use std::{fs, thread, time};

/// Argon One Fan Address for Raspberry Pi 4 B
const FAN_ADDRESS: u16 = 0x1a;
/// Minimum temperature for the fan to kick off (10)
const MIN_TEMP: f32 = 51.0;
/// Maximum temperature, this is when the fan hits full speed (100)
const MAX_TEMP: f32 = 69.0; // Nice

fn trim_newline(s: &mut String) {
    let len = s.trim_end_matches(&['\n'][..]).len();
    s.truncate(len);
}

fn get_temp() -> f32 {
    let mut value = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")
        .expect("Thermal zone 0 is not accessible");
    trim_newline(&mut value);
    let temp = value.parse::<f32>().unwrap() / 1000.0;
    return temp;
}

pub fn controller() {
    let mut bus = I2c::new().expect("No bus identified");
    bus.set_slave_address(FAN_ADDRESS)
        .expect("Unable to set the fan address");
    let bus = bus; // Remove the mut

    loop {
        let mut fan_speed: u8 = 0;
        let temp = get_temp();

        if temp >= MAX_TEMP {
            fan_speed = 100;
        } else if temp >= MIN_TEMP {
            fan_speed = (((temp - MIN_TEMP) * 90.0 / (MAX_TEMP - MIN_TEMP)) + 10.0) as u8; // Lineal bcz why not?
            if fan_speed > 100 {
                fan_speed = 100; // Shouldn't happen but I prefer to have fallbacks
            }
        }

        // println!("Setting fan_speed at {} for temp {}'c", fan_speed, temp);

        match bus.smbus_write_byte(0, fan_speed) {
            Ok(_) => thread::sleep(time::Duration::new(10, 0)),
            Err(e) => println!("Failed to set fan speed {}", e),
        }
    }
}
