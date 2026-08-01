use gpui::prelude::*;
use gpui::{
    App, Bounds, Context, FontWeight, QuitMode, Render, SharedString, TitlebarOptions, Window, WindowBounds, WindowKind, WindowOptions, actions, div, px, size,
};
use gpui_platform::application;
use pip_clock::{tabular_figures, Theme};
use std::time::{Duration, Instant};

actions!(clock, [ToggleTheme]);

struct Clock {
    time_text: SharedString,
    last_update: Instant,
    theme: Theme,
}

impl Clock {
    fn new() -> Self {
        Self {
            time_text: Self::current_time().into(),
            last_update: Instant::now(),
            theme: Theme::Dark,
        }
    }

    fn current_time() -> String {
        let now = chrono::Local::now();
        now.format("%H:%M:%S").to_string()
    }
}

impl Render for Clock {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.last_update.elapsed() >= Duration::from_secs(1) {
            self.time_text = Self::current_time().into();
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
            .size_full()
            .bg(bg)
            .child(
                div()
                    .text_3xl()
                    .font_weight(FontWeight::BOLD)
                    .font_features(tabular_figures())
                    .text_color(fg)
                    .child(self.time_text.clone()),
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
                    title: Some("PiP Clock".into()),
                    appears_transparent: true,
                    traffic_light_position: None,
                }),
                is_movable: true,
                is_resizable: true,
                ..Default::default()
            },
            |_, cx| cx.new(|_| Clock::new()),
        )
        .expect("failed to open window");
        cx.activate(true);
    });
}
