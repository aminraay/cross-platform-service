use std::io::{Error, ErrorKind, Write};
use std::time::Duration;

use dbus::blocking::Connection;

use crate::unix::dbus_systemd1::OrgFreedesktopSystemd1Manager;
use std::fs::{OpenOptions, remove_file};
use std::path::{PathBuf};

pub enum Mode {
    Replace,
    Fail,
    Isolate,
    IgnoreDependencies,
    IgnoreRequirements,
}

impl Into<&'static str> for Mode {
    fn into(self) -> &'static str {
        match self {
            Mode::Replace => "replace",
            Mode::Fail => "fail",
            Mode::Isolate => "isolate",
            Mode::IgnoreDependencies => "ignore-dependencies",
            Mode::IgnoreRequirements => "ignore-requirements",
        }
    }
}

pub fn start_unit(service_name: &str, mode: Mode, timeout: Duration) -> Result<(), Error> {
    let c = Connection::new_system()
        .map_err(|err| from_dbus_error(err))?;

    let p = c.with_proxy("org.freedesktop.systemd1",
                         "/org/freedesktop/systemd1", timeout);
    p.start_unit(service_name, mode.into())
        .map_err(|err| from_dbus_error(err))?;

    Ok(())
}

pub fn stop_unit(service_name: &str, mode: Mode, timeout: Duration) -> Result<(), Error> {
    let c = Connection::new_system()
        .map_err(|err| from_dbus_error(err))?;

    let p = c.with_proxy("org.freedesktop.systemd1",
                         "/org/freedesktop/systemd1", timeout);

    p.stop_unit(service_name, mode.into())
        .map_err(|err| from_dbus_error(err))?;

    Ok(())
}

fn from_dbus_error(err: dbus::Error) -> std::io::Error {
    std::io::Error::new(ErrorKind::Other, err.to_string())
}

pub fn install_string(file_name: &str, file_content: &str) -> Result<(), Error> {
    let mut file_path = PathBuf::from("/etc/systemd/system");
    file_path.push(file_name);

    if file_path.exists() {
        Err(Error::new(ErrorKind::Other, "File path already exists"))
    } else {
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)?;

        f.write(file_content.as_bytes())?;

        Ok(())
    }
}

pub fn delete_unit(service_name: &str) -> Result<(), Error> {
    let mut file_path = PathBuf::from("/etc/systemd/system");
    file_path.push(service_name);

    remove_file(file_path)
}

pub struct UnitStatus {
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub state: String,
    // Followed (5)
    pub exec_path: String,
}

pub fn get_unit_status(unit_name: &str, timeout: Duration) -> Result<Option<UnitStatus>, Error> {
    let c = Connection::new_system()
        .map_err(|err| from_dbus_error(err))?;

    let p = c.with_proxy("org.freedesktop.systemd1",
                         "/org/freedesktop/systemd1", timeout);

    let units = p.list_units_by_names(vec![unit_name])
        .map_err(|err| from_dbus_error(err))?;

    let unit = units.get(0)
        .unwrap();

    if unit.2.eq("not-found") {
        Ok(None)
    } else {
        let result = UnitStatus {
            name: (&unit.0).clone(),
            description: (&unit.1).clone(),
            is_active: (unit.3.eq("active")),
            state: (&unit.4).to_string(),
            exec_path: (&unit.6).to_string(),
        };

        Ok(Some(result))
    }
    // Ok(())
}

pub fn disable_unit_files(files: Vec<&str>, timeout: Duration) -> Result<(), Error> {
    let c = Connection::new_system()
        .map_err(|err| from_dbus_error(err))?;

    let p = c.with_proxy("org.freedesktop.systemd1",
                         "/org/freedesktop/systemd1", timeout);
    p.disable_unit_files(files, true)
        .map_err(|err| from_dbus_error(err))?;

    Ok(())
}

pub fn enable_unit_files(files: Vec<&str>, timeout: Duration) -> Result<(), Error> {
    let c = Connection::new_system()
        .map_err(|err| from_dbus_error(err))?;

    let p = c.with_proxy("org.freedesktop.systemd1",
                         "/org/freedesktop/systemd1", timeout);
    p.enable_unit_files(files, true, true)
        .map_err(|err| from_dbus_error(err))?;

    Ok(())
}