#[cfg(unix)]
use std::fs::OpenOptions;
#[cfg(unix)]
use std::io::Write;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(unix)]
use signal_hook::{consts::SIGTERM, iterator::Signals};

#[cfg(windows)]
use crate::windows::win32_api_services::{
    register_service_ctrl_handler,
    ServiceControl,
    ServiceControlsAccepted,
    ServiceState,
    ServiceStatus,
    ServiceType,
    set_service_status,
    start_service_ctrl_dispatcher,
};


/// A function structure to call as service main function
///
/// Argument indicate if service is running or stopped by operating system. The function should
/// store the state and return when argument value became false
pub type ServiceMainFunc = fn(Arc<AtomicBool>);

/// Call specified function and wait for SIGTERM in another thread
///
#[cfg(unix)]
pub fn start_service(service_name: &str, service_main: ServiceMainFunc) {
    let running = Arc::new(AtomicBool::new(true));

    let mut signals = Signals::new(&[SIGTERM]).unwrap();
    let th_running = running.clone();

    std::thread::spawn(move || {
        for _ in signals.forever() {
            th_running.store(false, Ordering::Relaxed);
        }
    });

    service_main(running.clone());
}

/// Initialize current process as service then call specified function
///
/// Initialize service by calling [StartServiceCtrlDispatcherW](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-startservicectrldispatcherw)
///
#[cfg(windows)]
pub fn start_service(service_name: &str, main_func: ServiceMainFunc) {
    unsafe {
        RUNNING = Some(Arc::new(AtomicBool::new(true)));
        MAIN_FUNC = Some(main_func);
        SERVICE_NAME = Some(service_name.into());
    }

    start_service_ctrl_dispatcher(service_name, Some(svc_main)).unwrap()
}

#[cfg(windows)]
static mut RUNNING: Option<Arc<AtomicBool>> = None;
#[cfg(windows)]
static mut MAIN_FUNC: Option<ServiceMainFunc> = None;
#[cfg(windows)]
static mut SERVICE_NAME: Option<String> = None;

#[cfg(windows)]
fn svc_main(_: Vec<String>) {
    let service_name = unsafe {
        SERVICE_NAME.as_ref().unwrap().clone()
    };

    let handle = register_service_ctrl_handler(
        service_name.as_str(), Some(svc_ctrl)).unwrap();

    set_service_status(&handle, ServiceStatus {
        service_type: ServiceType::Win32OwnProcess,
        service_state: ServiceState::Running,
        controls_accepted: vec![ServiceControlsAccepted::Stop],
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint: 0,
    }).expect("Cannot set service status");

    unsafe {
        if MAIN_FUNC.is_some() {
            MAIN_FUNC.as_ref().unwrap()(RUNNING.as_ref().unwrap().clone());
        }
    }

    set_service_status(&handle, ServiceStatus {
        service_type: ServiceType::Win32OwnProcess,
        service_state: ServiceState::Stopped,
        controls_accepted: vec![],
        win32_exit_code: 0,
        service_specific_exit_code: 0,
        check_point: 0,
        wait_hint: 0,
    }).expect("Cannot set service status");
}

#[cfg(windows)]
fn svc_ctrl(control: ServiceControl) {
    // Handle service controls sent by operating system
    // Note that the list of accepted controls must be set by set_service_status in svc_main
    // function
    match control {
        ServiceControl::Stop => {
            unsafe {
                RUNNING.as_ref().unwrap().store(false, Ordering::Relaxed);
            }
        }
        _ => {}
    }
}
