use std::string;

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
