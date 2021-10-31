use std::io::Error;

use crate::windows::win32_api_services::*;

/// Provides needed information to install a windows service
///
/// For more information visit [CreateServiceW](https://docs.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-createservicew)
pub struct WindowsServiceInfo {
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub desired_access: u32,
    pub service_type: ServiceType,
    pub start_type: ServiceStartType,
    pub error_control: ServiceErrorControl,
    pub binary_path: String,
    pub load_order_group: Option<String>,
    pub tag_id: Option<u32>,
    pub dependencies: Option<String>,
    pub service_start_name: Option<String>,
    pub password: Option<String>,
}

/// Starts specified Windows service
pub fn start_windows_service(service_name: &str) -> Result<(), Error> {
    let sc_handle = open_service_manager()?;

    match open_service(&sc_handle, service_name) {
        Ok(s_handle) => {
            let result = start_service(&s_handle);

            close_service_handle(&s_handle);
            close_service_handle(&sc_handle);

            result
        }
        Err(err) => {
            close_service_handle(&sc_handle);

            Err(err)
        }
    }
}

/// Sends control value to specified Windows service
pub fn control_windows_service(service_name: &str, control: ServiceControl) -> Result<(), Error> {
    let sc_handle = open_service_manager()?;

    match open_service(&sc_handle, service_name) {
        Ok(s_handle) => {
            let result = control_service(&s_handle, control);

            close_service_handle(&s_handle);
            close_service_handle(&sc_handle);

            result
        }
        Err(err) => {
            close_service_handle(&sc_handle);

            Err(err)
        }
    }
}

/// Get current status of specified Windows service
pub fn get_service_status(service_name: &str) -> Result<ServiceStatus, Error> {
    let sc_handle = open_service_manager()?;

    match open_service(&sc_handle, service_name) {
        Ok(s_handle) => {
            let result = query_service_status(&s_handle);

            close_service_handle(&s_handle);
            close_service_handle(&sc_handle);

            result
        }
        Err(err) => {
            close_service_handle(&sc_handle);

            Err(err)
        }
    }
}

/// Install new service to Windows from specified information
pub fn install_windows_service(info: WindowsServiceInfo) -> Result<(), Error> {
    let sc_handle = open_service_manager()?;

    match create_service(&sc_handle,
                         info.name.as_str(),
                         info.display_name.as_str(),
                         info.desired_access,
                         info.service_type,
                         info.start_type,
                         info.error_control,
                         info.binary_path.as_str(),
                         info.load_order_group,
                         info.tag_id,
                         info.dependencies,
                         info.service_start_name,
                         info.password) {
        Ok(h) => {
            let result = if info.description.is_some() {
                let s_handle = open_service(&sc_handle, info.name.as_str())?;
                let r = change_service_config_description(&s_handle,
                                                          info.description.unwrap().as_str());
                    // change_service_config2(&s_handle, ServiceConfig::Description,
                    //                            ServiceConfigValue::Text(info.description.unwrap()));
                close_service_handle(&s_handle);
                r
            } else {
                Ok(())
            };

            close_service_handle(&h);
            close_service_handle(&sc_handle);

            result
        }
        Err(err) => {
            close_service_handle(&sc_handle);
            Err(err)
        }
    }
}

/// Delete specified service
pub fn delete_windows_service(service_name: &str) -> Result<(), Error> {
    let sc_handle = open_service_manager()?;
    match open_service(&sc_handle, service_name) {
        Ok(s_handle) => {
            match delete_service(&s_handle) {
                Ok(_) => {
                    Ok(())
                }
                Err(err) => {
                    close_service_handle(&s_handle);
                    close_service_handle(&sc_handle);
                    Err(err)
                }
            }
        }
        Err(err) => {
            close_service_handle(&sc_handle);
            Err(err)
        }
    }
}
