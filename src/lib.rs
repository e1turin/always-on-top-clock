use gpui::{rgb, FontFeatures, Hsla};
use std::sync::Arc;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

/// Enables the OpenType `tnum` (tabular figures) feature, making all digits
/// equally wide so clock numbers don't shift when digits change.
pub fn tabular_figures() -> FontFeatures {
    FontFeatures(Arc::new(vec![("tnum".into(), 1)]))
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dark
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
