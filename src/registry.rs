use std::os::windows::prelude::OsStrExt;
use std::{env, ffi};
use windows::core::PCWSTR;
use windows::Win32::System::Registry::{
    RegOpenKeyExW, RegSetKeyValueW, HKEY, HKEY_LOCAL_MACHINE, KEY_READ, REG_SZ,
};

use crate::error::{format_win_error, RegistryError};
use crate::EventLogKey;

const EVENT_LOG_REG_BASE: &str = r"SYSTEM\CurrentControlSet\Services\EventLog";

/// Sets EventMessageFile registry key to point to current executable.
/// Requires admin rights.
pub fn set_message_file_location(key: &EventLogKey, source: &str) -> Result<(), RegistryError> {
    let mut subkey: ffi::OsString = ffi::OsString::from(EVENT_LOG_REG_BASE);
    subkey.push("\0");
    let subkey_char_seq: Vec<u16> = subkey.as_os_str().encode_wide().collect();

    let mut subkey_handle = HKEY::default();

    let open_result = unsafe {
        RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(subkey_char_seq.as_ptr()),
            0,
            KEY_READ,
            &mut subkey_handle,
        )
    };

    if open_result.is_err() {
        let error = format_win_error(open_result);
        return Err(RegistryError::FailedToOpen(
            error.unwrap_or("failed to format error".to_owned()),
        ));
    }

    let os_string_path: ffi::OsString = env::current_exe().unwrap().into();
    let mut path: Vec<u16> = os_string_path.as_os_str().encode_wide().collect();
    path.push(0);

    let value_name: Vec<u16> = ffi::OsString::from("EventMessageFile\0")
        .as_os_str()
        .encode_wide()
        .collect();

    let mut key_and_source: Vec<u16> =
        ffi::OsString::from(format!("{}\\{}", key.to_string(), source))
            .as_os_str()
            .encode_wide()
            .collect();
    key_and_source.push(0);

    let result = unsafe {
        RegSetKeyValueW(
            subkey_handle,
            PCWSTR(key_and_source.as_ptr()),
            PCWSTR(value_name.as_ptr()),
            REG_SZ.0,
            path.as_ptr() as _,
            path.len() as u32 * 2,
        )
    };

    if result.is_err() {
        let error = format_win_error(result);
        return Err(RegistryError::FailedToSet(
            error.unwrap_or("failed to format error".to_owned()),
        ));
    }
    Ok(())
}
