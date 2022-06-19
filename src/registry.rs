use std::os::windows::prelude::OsStrExt;
use std::{env, ffi};
use windows::core::PCWSTR;
use windows::Win32::System::Registry::{
    RegOpenKeyExW, RegSetKeyValueW, HKEY, HKEY_LOCAL_MACHINE, KEY_READ, REG_SZ,
};

use crate::EventLogKey;
use crate::EVENT_LOG_REG_BASE;

pub fn set_message_file_location(key: EventLogKey, source: &str) {
    let mut subkey: ffi::OsString = ffi::OsString::from(EVENT_LOG_REG_BASE);
    subkey.push("\0");
    let subkey_char_seq: Vec<u16> = subkey.as_os_str().encode_wide().collect();

    println!("{:?}", subkey);

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
        println!("open reg key failed: {}", open_result.0)
    }

    let os_string_path: ffi::OsString = env::current_exe().unwrap().into();
    println!("{:?}", os_string_path);
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
    println!("{:?}", path);
    println!("{:?}", value_name);
    println!("{:?}", key_and_source);

    let result = unsafe {
        println!("{}", path.len());

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
        println!("Error setting the key: {}", result.0)
    }
}
