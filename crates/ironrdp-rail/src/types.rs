//! Common types and structures used throughout the RAIL protocol

use num_derive::{FromPrimitive, ToPrimitive};

/// Window ID type used in RAIL protocol
pub type WindowId = u32;

/// Rectangle structure for window positioning and sizing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rectangle {
    pub left: i16,
    pub top: i16,
    pub right: i16,
    pub bottom: i16,
}

impl Rectangle {
    pub fn new(left: i16, top: i16, right: i16, bottom: i16) -> Self {
        Self { left, top, right, bottom }
    }
    
    pub fn width(&self) -> u16 {
        (self.right - self.left) as u16
    }
    
    pub fn height(&self) -> u16 {
        (self.bottom - self.top) as u16
    }
}

/// Window state flags as defined in MS-RDPERP
#[derive(Debug, Clone, Copy, PartialEq, Eq, FromPrimitive, ToPrimitive)]
pub enum WindowState {
    /// The window is in a restored state
    Restored = 0x00,
    /// The window is minimized
    Minimized = 0x01,
    /// The window is maximized
    Maximized = 0x02,
}

/// Window show state flags
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct WindowShowState: u32 {
        /// Hide the window
        const HIDE = 0x0000_0000;
        /// Show the window in normal state
        const SHOW_NORMAL = 0x0000_0001;
        /// Show the window minimized
        const SHOW_MINIMIZED = 0x0000_0002;
        /// Show the window maximized
        const SHOW_MAXIMIZED = 0x0000_0003;
        /// Show the window without activating it
        const SHOW_NO_ACTIVATE = 0x0000_0004;
        /// Show the window and activate it
        const SHOW = 0x0000_0005;
        /// Minimize the window and activate the next top-level window
        const MINIMIZE = 0x0000_0006;
        /// Show the window minimized without activating it
        const SHOW_MIN_NO_ACTIVE = 0x0000_0007;
        /// Show the window in its current state without activating it
        const SHOW_NA = 0x0000_0008;
        /// Restore and activate the window
        const RESTORE = 0x0000_0009;
    }
}

/// Execute flags for launching applications
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ExecuteFlags: u32 {
        /// Execute the application normally
        const NORMAL = 0x0000_0000;
        /// Execute the application with elevated privileges
        const ELEVATED = 0x0000_0001;
        /// Execute as a service
        const AS_SERVICE = 0x0000_0002;
        /// Execute with environment variables from the calling process
        const INHERIT_ENVIRONMENT = 0x0000_0004;
    }
}

/// Language bar status flags
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct LanguageBarStatus: u32 {
        /// Language bar is enabled
        const ENABLED = 0x0000_0001;
        /// Language bar is visible
        const VISIBLE = 0x0000_0002;
        /// Language bar is docked
        const DOCKED = 0x0000_0004;
        /// Language bar supports multi-language
        const MULTI_LANGUAGE = 0x0000_0008;
    }
}