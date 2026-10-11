use gpui::{rgb, FontFeatures, Hsla, Window};
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

/// Hands window movement over to the compositor / window manager. On Linux,
/// `WindowControlArea::Drag` is ignored, so dragging must be started manually;
/// Windows and macOS already handle it via the control area.
pub fn start_window_drag(_window: &Window) {
    #[cfg(any(target_os = "linux", target_os = "freebsd"))]
    _window.start_window_move();
}

/// Enables the OpenType `tnum` (tabular figures) feature, making all digits
/// equally wide so clock numbers don't shift when digits change.
pub fn tabular_figures() -> FontFeatures {
    FontFeatures(Arc::new(vec![("tnum".into(), 1)]))
}

impl Default for Theme {
    fn default() -> Self {
        #[cfg(feature = "light-theme")]
        {
            Theme::Light
        }

        #[cfg(not(feature = "light-theme"))]
        {
            Theme::Dark
        }
    }
}

impl Theme {
    pub fn toggle(&mut self) {
        *self = match self {
            Theme::Dark => Theme::Light,
            Theme::Light => Theme::Dark,
        };
    }

    pub fn bg(self) -> Hsla {
        match self {
            Theme::Dark => rgb(0x000000).into(),
            Theme::Light => rgb(0xffffff).into(),
        }
    }

    pub fn fg(self) -> Hsla {
        match self {
            Theme::Dark => rgb(0xffffff).into(),
            Theme::Light => rgb(0x000000).into(),
        }
    }

    pub fn muted(self) -> Hsla {
        match self {
            Theme::Dark => rgb(0x666666).into(),
            Theme::Light => rgb(0x999999).into(),
        }
    }

    pub fn dim(self) -> Hsla {
        match self {
            Theme::Dark => rgb(0x222222).into(),
            Theme::Light => rgb(0xdddddd).into(),
        }
    }
}
