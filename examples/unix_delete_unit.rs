use cross_platform_service::unix::services::delete_unit;

const SERVICE_NAME: &str = "MyUnixSVC.service";

fn main() {
    match delete_unit(SERVICE_NAME) {
        Ok(_) => {
            println!("Service '{}' deleted successfully", SERVICE_NAME);
        }
        Err(err) => {
            println!("Could not delete service '{}', {}", SERVICE_NAME, err.to_string());
        }
    }
}