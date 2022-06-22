use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::PSID;
use windows::Win32::System::EventLog::{
    DeregisterEventSource, EventSourceHandle, RegisterEventSourceW, ReportEventW, REPORT_EVENT_TYPE,
};

use std::fmt;
use std::ptr;

mod builder;
mod error;
mod registry;

include!(concat!(env!("OUT_DIR"), "/messages.rs"));

#[derive(Debug)]
pub enum EventLogKey {
    Application,
    /// Using Security will always fail
    Security,
    System,
    Custom(String),
}

impl fmt::Display for EventLogKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Custom(key) => write!(f, "{}", key),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Set Application as default for EventLogKey since it's the default in windows
impl Default for EventLogKey {
    fn default() -> Self {
        EventLogKey::Application
    }
}

#[derive(thiserror::Error, Debug)]
pub enum EventLogError {
    #[error("Couldn't register/open event source: {0}")]
    RegisterFailed(#[from] windows::core::Error),
    #[error("Logger can only be initalized once")]
    InitalizationFailed(#[from] log::SetLoggerError),
    #[error("Failed to set message file registry entry")]
    RegistryError(#[from] error::RegistryError),
}

pub struct EventLog {
    handle: EventSourceHandle,
    level: log::Level,
}

impl EventLog {
    pub fn init(
        key: EventLogKey,
        source: &str,
        level: log::Level,
    ) -> std::result::Result<(), EventLogError> {
        // Set necessary reg key
        registry::set_message_file_location(key, source)?;

        let event_source: Vec<u16> = str::encode_utf16(&format!("{}\0", source)).collect();
        let handle =
            unsafe { RegisterEventSourceW(PCWSTR(ptr::null()), PCWSTR(event_source.as_ptr()))? };

        let logger = Self {
            handle: handle,
            level: level,
        };
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(log::LevelFilter::Trace);
        Ok(())
    }

    pub fn builder() -> builder::EventLogBuilder {
        builder::EventLogBuilder::default()
    }

    pub fn register_event_source(source: &str) -> Result<EventSourceHandle, EventLogError> {
        let mut source_char_seq = str::encode_utf16(source).collect::<Vec<u16>>();
        source_char_seq.push(0);
        let handle =
            unsafe { RegisterEventSourceW(PCWSTR(ptr::null()), PCWSTR(source_char_seq.as_ptr()))? };
        Ok(handle)
    }
}

impl log::Log for EventLog {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= self.level
    }
    /// Flush is no-op since this logger doesn't hold any buffers
    fn flush(&self) {}

    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let message = format!("{}\0", record.args());
        let mut code_points: Vec<u16> = message.encode_utf16().collect();
        code_points.push(0);

        let type_and_id = match record.level() {
            log::Level::Error => (EventLogType::Error, ERROR),
            log::Level::Warn => (EventLogType::Warning, WARNING),
            log::Level::Info => (EventLogType::Information, INFO),
            log::Level::Debug => (EventLogType::Information, DEBUG),
            log::Level::Trace => (EventLogType::Information, TRACE),
        };

        let success = unsafe {
            ReportEventW(
                self.handle,
                REPORT_EVENT_TYPE(type_and_id.0 as u16),
                0,
                type_and_id.1,
                PSID(ptr::null_mut()),
                0,
                &[PWSTR(code_points.as_mut_ptr())],
                ptr::null(),
            )
            .as_bool()
        };

        if !success {
            // Should the logger panic if it fails to log?
            panic!("Writing log entry failed");
        }
    }
}

impl Drop for EventLog {
    fn drop(&mut self) {
        let success = unsafe { DeregisterEventSource(self.handle).as_bool() };

        if !success {
            // Does panicing here make sense?
            panic!("Deregistering Event Log handle failed");
        }
    }
}

pub enum EventLogType {
    Success = 0x0000,
    AuditFailure = 0x0010,
    AuditSuccess = 0x0008,
    Error = 0x0001,
    Information = 0x0004,
    Warning = 0x0002,
}

#[test]
fn log_to_event_log() {
    EventLog::init(
        EventLogKey::Custom("AAPPLICATION".to_string()),
        "AAATEST",
        log::Level::Trace,
    )
    .expect("init failed");
    //log::info!("Test log")
}

#[test]
fn event_log_key_display() {
    assert_eq!(EventLogKey::Application.to_string(), "Application");
    assert_eq!(EventLogKey::Security.to_string(), "Security");
    assert_eq!(EventLogKey::System.to_string(), "System");
    assert_eq!(
        EventLogKey::Custom("Custom".to_string()).to_string(),
        "Custom"
    );
    assert_eq!(
        EventLogKey::Custom("WindowsEventLog".to_string()).to_string(),
        "WindowsEventLog"
    );
}
