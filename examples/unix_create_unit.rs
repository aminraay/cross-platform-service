use cross_platform_service::unix::services::install_string;
use std::io::Error;
use std::env::current_exe;

const SERVICE_NAME: &str = "MyUnixSVC.service";

fn main() {
    let mut path = current_exe().unwrap();
    path.pop();
    path.push("cross_platform_service");

    // For more information about unit file visit following url
    // https://www.digitalocean.com/community/tutorials/understanding-systemd-units-and-unit-files
    let service_config = format!("[Unit]
Description=My unix service description

[Service]
Type=simple
ExecStart={}", path.to_str().unwrap());

    match install_string(SERVICE_NAME, service_config.as_str()) {
        Ok(_) => {
            println!("Service '{}' installed successfully", SERVICE_NAME);
        }
        Err(err) => {
            println!("Service '{}' could not be installed. {}", SERVICE_NAME, err.to_string());
        }
    }
}
