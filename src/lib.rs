use windows::core::{PCWSTR, PWSTR};
use windows::Win32::Foundation::PSID;
use windows::Win32::System::EventLog::{
    DeregisterEventSource, EventSourceHandle, RegisterEventSourceW, ReportEventW, REPORT_EVENT_TYPE,
};

use std::{ptr, string};

mod error;
mod messages;
mod registry;

/// The key under which events get logged in.
///
/// Most programs should use either the default Application log or create a
/// custom log.
///
/// [More info in Win32 documentation]([https://docs.microsoft.com/en-us/windows/win32/eventlog/event-sources)
#[derive(Debug)]
pub enum EventLogKey {
    /// Application log is the default and most programs should be using it
    Application,
    /// Security is reserved for Windows internals only so using it will always fail
    Security,
    /// Device drivers should should add their sources to System log
    System,
    /// Custom logs can be created for application or services
    Custom(String),
}

impl string::ToString for EventLogKey {
    fn to_string(&self) -> String {
        match self {
            Self::Custom(key) => key.into(),
            _ => format!("{:?}", self),
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

struct InnerLogger {
    handle: EventSourceHandle,
    level: log::Level,
}

/// Data struct used to configure and register the event source
pub struct EventLog {
    level: log::Level,
    source: String,
    event_log_key: EventLogKey,
}

impl EventLog {
    /// Constructs a new EventLog
    pub fn new(key: EventLogKey, source: impl Into<String>, level: log::Level) -> Self {
        Self {
            level,
            source: source.into(),
            event_log_key: key,
        }
    }

    /// Sets the `EventMessageFile` registry key to point to current executable
    ///
    /// Requires admin privileges
    /// Calling this function is only necessary once on install, if the executable doesn't move.
    /// Programs which need to be run as Administrator anyways may call this function every time they register their logger
    pub fn set_message_file_location(self) -> Result<Self, Box<dyn std::error::Error>> {
        registry::set_message_file_location(&self.event_log_key, &self.source)?;
        Ok(self)
    }

    /// Registers the event source, and sets up the logger
    pub fn register(self) -> Result<Self, Box<dyn std::error::Error>> {
        let handle = Self::register_event_source(&self.source)?;

        let logger = InnerLogger {
            handle,
            level: self.level,
        };
        log::set_boxed_logger(Box::new(logger))?;
        log::set_max_level(log::LevelFilter::Trace);
        Ok(self)
    }

    /// Calls the win32 (RegisterEventSourceW)[https://docs.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-registereventsourcew] function with the configured source
    fn register_event_source(source: &str) -> Result<EventSourceHandle, EventLogError> {
        let mut source_char_seq = str::encode_utf16(source).collect::<Vec<u16>>();
        source_char_seq.push(0);
        let handle =
            unsafe { RegisterEventSourceW(PCWSTR(ptr::null()), PCWSTR(source_char_seq.as_ptr()))? };
        Ok(handle)
    }
}

impl log::Log for InnerLogger {
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
            log::Level::Error => (EventLogType::Error, messages::ERROR),
            log::Level::Warn => (EventLogType::Warning, messages::WARNING),
            log::Level::Info => (EventLogType::Information, messages::INFO),
            log::Level::Debug => (EventLogType::Information, messages::DEBUG),
            log::Level::Trace => (EventLogType::Information, messages::TRACE),
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

impl Drop for InnerLogger {
    fn drop(&mut self) {
        let success = unsafe { DeregisterEventSource(self.handle).as_bool() };

        if !success {
            // Does panicing here make sense?
            panic!("Deregistering Event Log handle failed");
        }
    }
}

#[allow(dead_code)]
enum EventLogType {
    Success = 0x0000,
    AuditFailure = 0x0010,
    AuditSuccess = 0x0008,
    Error = 0x0001,
    Information = 0x0004,
    Warning = 0x0002,
}

#[test]
fn log_to_event_log() {
    EventLog::new(
        EventLogKey::Custom("AAPPLICATION".to_string()),
        "AAATEST",
        log::Level::Trace,
    )
    .set_message_file_location()
    .unwrap()
    .register()
    .unwrap();
    log::info!("Test log")
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
