use cross_platform_service::service_manager::stop;

const SERVICE_NAME: &str = "MyCrPlSVC";

fn main() {
    match stop(SERVICE_NAME) {
        Ok(_) => {
            println!("Service '{}' successfully stopped", SERVICE_NAME)
        }
        Err(err) => {
            println!("Could not stop service '{}', {}", SERVICE_NAME, err.to_string())
        }
    }
}