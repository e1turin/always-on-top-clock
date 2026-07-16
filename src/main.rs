//! PiP Clock - Always-on-top clock widget built with GPUI (Zed's GPU UI framework)

use gpui::prelude::*;
use gpui::{
    App, Bounds, Context, Entity, Render, SharedString, Window, WindowBounds, WindowKind,
    WindowOptions, div, px, rgb, size, FontWeight, TitlebarOptions, KeyBinding,
};
use gpui::actions;
use gpui_platform::application;
use std::time::{Duration, Instant};

actions!(clock, [Quit]);

struct Clock {
    time_text: SharedString,
    last_update: Instant,
}

impl Clock {
    fn new() -> Self {
        Self {
            time_text: Self::current_time().into(),
            last_update: Instant::now(),
        }
    }

    fn current_time() -> String {
        let now = chrono::Local::now();
        now.format("%H:%M").to_string()
    }
}

impl Render for Clock {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Update time every minute
        if self.last_update.elapsed() >= Duration::from_secs(60) {
            self.time_text = Self::current_time().into();
            self.last_update = Instant::now();
            cx.notify();
        }

        // Request next frame for smooth updates
        window.request_animation_frame();

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(rgb(0x000000))
            .text_color(rgb(0xffffff))
            .rounded_md()
            .p_4()
            .child(
                div()
                    .text_3xl()
                    .font_weight(FontWeight::BOLD)
                    .child(self.time_text.clone()),
            )
    }
}

struct ClockApp {
    clock: Entity<Clock>,
}

impl ClockApp {
    fn new(cx: &mut App) -> Self {
        let clock = cx.new(|_| Clock::new());
        Self { clock }
    }
}

impl Render for ClockApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0x000000))
            .child(self.clock.clone())
            .on_action(cx.listener(|_, _: &Quit, _, cx| cx.quit()))
    }
}

fn main() {
    application().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(220.0), px(100.0)), cx);

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
                is_resizable: false,
                ..Default::default()
            },
            |_, cx| cx.new(|cx| ClockApp::new(cx)),
        )
        .unwrap();

        cx.activate(true);
        cx.on_action(|_: &Quit, cx| cx.quit());
        cx.bind_keys([KeyBinding::new("cmd-q", Quit, None)]);
    });
}
