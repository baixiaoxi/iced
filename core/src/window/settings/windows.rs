//! Platform specific settings for Windows.
use raw_window_handle::RawWindowHandle;

/// The platform specific window settings of an application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformSpecific {
    /// Parent window
    pub parent: Option<RawWindowHandle>,

    /// Drag and drop support
    pub drag_and_drop: bool,

    /// Whether show or hide the window icon in the taskbar.
    pub skip_taskbar: bool,

    /// Window shadow
    pub undercorated_shadow: bool,
}

impl Default for PlatformSpecific {
    fn default() -> Self {
        Self {
            parent: None,
            drag_and_drop: true,
            skip_taskbar: false,
            undercorated_shadow: false,
        }
    }
}
