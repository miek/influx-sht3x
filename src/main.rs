extern crate linux_embedded_hal as hal;
#[macro_use]
extern crate influx_db_client;
extern crate sht3x;
extern crate sys_info;

use hal::{Delay, I2cdev};
use influx_db_client::{Client, Point, Points, Value, Precision};
use sht3x::{SHT3x, Address, Repeatability};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let client = Client::new(&args[1], &args[2]);

    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut sensor = SHT3x::new(dev, Delay, Address::Low);
    let m = sensor.measure(Repeatability::High).unwrap();

    let mut point = point!("test");
    point.add_timestamp(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64);
    point.add_field("temperature", Value::Float(m.temperature as f64 / 100.0));
    point.add_field("humidity", Value::Float(m.humidity as f64 / 100.0));
    point.add_tag("hostname", Value::String(sys_info::hostname().unwrap()));

    let points = points!(point);

    let _ = client.write_points(points, Some(Precision::Seconds), None).unwrap();
}
