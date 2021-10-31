# cross-platform-service

The "cross-platform-service" crate lets you developing cross-platform services and service managers for Windows, Linux, and macOS.
It supports install, delete, start and stop.

On Windows, API is called by using the [microsoft/windows-rs](https://github.com/microsoft/windows-rs) crate, 
And it is also possible to call the Windows API directory if the service has special needs for Windows.

On Linux, D-Bus used for communication with systemd. The default behavior is defining a systemd service unit, 
in "/etc/systemd/system" path.

## Getting started

```toml

```

```rust
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use service_manager_rs::service::start_service;

const SERVICE_NAME: &str = "MyCrPlSVC";

fn main() {
    start_service(SERVICE_NAME, service_main);
}

fn service_main(running: Arc<AtomicBool>) {
    // The following code will run service for a minute and check if service stopped every
    // 100 milli-seconds
    // Return from this function will stop service
    for _ in 0..600 {
        if !running.load(Ordering::Relaxed) {
            // Write stopping service codes here
            // In linux SIGTERM signal set running to false and in Windows service stop command
            // will do the same

            return;
        }

        std::thread::sleep(Duration::from_millis(100));
    }
}
```

To compile the project on Linux, D-Bus developing libraries are required, which is may be installed with the following
command:

```shell
apt install libdbus-1-dev
```

## Examples

### cross-platform

[Install a cross-platform Service](examples/cross_platform_install.rs)

[Delete a cross-platform service](examples/cross_platform_delete.rs)

[Start a cross-platform service](examples/cross_platform_start.rs)

[Stop a cross-platform service](examples/cross_platform_stop.rs)

### Windows Services

[Install Windows service](examples/win_create_service.rs)

