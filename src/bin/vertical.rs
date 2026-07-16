use gpui::prelude::*;
use gpui::{
    actions, div, px, rgb, size, App, Bounds, Context, Entity, FontWeight, KeyBinding,
    Render, SharedString, TitlebarOptions, Window, WindowBounds, WindowKind, WindowOptions,
};
use gpui_platform::application;
use pip_clock::Theme;
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
            theme: Theme::Dark,
        }
    }

    fn current_hm() -> (String, String) {
        let now = chrono::Local::now();
        (
            now.format("%H").to_string(),
            now.format("%M").to_string(),
        )
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
                    .text_size(px(100.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(fg)
                    .child(self.hours.clone()),
            )
            .child(
                div()
                    .text_size(px(100.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(fg)
                    .child(self.minutes.clone()),
            )
            .on_action(cx.listener(|this: &mut VerticalClock, _: &ToggleTheme, _, cx| {
                this.theme.toggle();
                cx.notify();
            }))
    }
}

fn main() {
    application().run(|cx: &mut App| {
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
                is_resizable: false,
                ..Default::default()
            },
            |_, cx| cx.new(|_| VerticalClock::new()),
        )
        .unwrap();

        cx.activate(true);
        cx.on_action(|_: &ToggleTheme, _cx| {});
        cx.bind_keys([KeyBinding::new("t", ToggleTheme, None)]);
    });
}
