use cross_platform_service::unix::services::{start_unit, Mode};
use std::time::Duration;
use std::io::Error;

const SERVICE_NAME: &str = "MyUnixSVC.service";

fn main() {
    match start_unit(SERVICE_NAME,
                     Mode::Replace,
                     Duration::from_secs(2)) {
        Ok(_) => {
            println!("Service '{}' started successfully", SERVICE_NAME);
        }
        Err(err) => {
            println!("Could not start service '{}', {}", SERVICE_NAME, err.to_string());
        }
    }
}