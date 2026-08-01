use gpui::prelude::*;
use gpui::{
    actions, div, px, size, App, Bounds, Context, FontWeight, QuitMode, Render,
    SharedString, TitlebarOptions, Window, WindowBounds, WindowKind, WindowOptions,
};
use gpui_platform::application;
use pip_clock::{tabular_figures, Theme};
use std::time::{Duration, Instant};

actions!(vertical, [ToggleTheme]);

struct VerticalClock {
    hours: SharedString,
    minutes: SharedString,
    last_update: Instant,
    theme: Theme,
}

impl VerticalClock {
    fn new() -> Self {
        let (h, m) = Self::current_hm();
        Self {
            hours: h.into(),
            minutes: m.into(),
            last_update: Instant::now(),
            theme: Default::default(),
        }
    }

    fn current_hm() -> (String, String) {
        let now = chrono::Local::now();
        (now.format("%H").to_string(), now.format("%M").to_string())
    }
}

impl Render for VerticalClock {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.last_update.elapsed() >= Duration::from_secs(1) {
            let (h, m) = Self::current_hm();
            self.hours = h.into();
            self.minutes = m.into();
            self.last_update = Instant::now();
            cx.notify();
        }
        window.request_animation_frame();

        let bg = self.theme.bg();
        let fg = self.theme.fg();

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_0()
            .size_full()
            .bg(bg)
            .child(
                div()
                    .text_size(px(50.0))
                    .font_weight(FontWeight::BOLD)
                    .font_features(tabular_figures())
                    .text_color(fg)
                    .child(
                        self.hours.clone()
                    ),
            )
            .child(
                div()
                    .text_size(px(50.0))
                    .font_weight(FontWeight::BOLD)
                    .font_features(tabular_figures())
                    .text_color(fg)
                    .child(self.minutes.clone()),
            )
    }
}

fn main() {
    application().with_quit_mode(QuitMode::LastWindowClosed).run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                kind: WindowKind::PopUp,
                titlebar: Some(TitlebarOptions {
                    title: Some("PiP Vertical".into()),
                    appears_transparent: true,
                    traffic_light_position: None,
                }),
                is_movable: true,
                is_resizable: true,
                ..Default::default()
            },
            |_, cx| cx.new(|_| VerticalClock::new()),
        )
        .expect("failed to open window");
        cx.activate(true);
    });
}
