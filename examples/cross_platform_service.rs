use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use cross_platform_service::service::start_service;

const SERVICE_NAME: &str = "MyCrPlSVC";

fn main() {
    // By calling service_start, "service_main" will be called as the service function,
    // and current function will be continued after returning from service_main
    start_service(SERVICE_NAME, service_main);
}

fn service_main(running: Arc<AtomicBool>) {
    // The following code will run service for a minute and check if service stopped by operating-
    // system every 100 milli-seconds
    // Returning from this function will notify operating system that current service has
    // been stopped
    for _ in 0..600 {
        if !running.load(Ordering::Relaxed) {
            // Running value will be set to false when SIGTERM signal received on Linux or stop
            // control sent by Windows service manager

            return;
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}