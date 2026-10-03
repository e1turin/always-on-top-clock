use gpui::prelude::*;
use gpui::{
    actions, div, point, px, size, App, Bounds, Context, FocusHandle, FontWeight, KeyBinding, Menu,
    MenuItem, QuitMode, Render, SharedString, TitlebarOptions, Window, WindowBounds, WindowKind,
    WindowOptions,
};
use gpui_platform::application;
use pip_clock::{tabular_figures, Theme};
use std::time::{Duration, Instant};

actions!(vertical, [ToggleTheme, NewWindow, CloseWindow, Quit]);

struct VerticalClock {
    focus_handle: FocusHandle,
    hours: SharedString,
    minutes: SharedString,
    last_update: Instant,
    theme: Theme,
}

impl VerticalClock {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);
        let (h, m) = Self::current_hm();

        Self {
            focus_handle,
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
            .on_action(|_: &CloseWindow, window, _| window.remove_window())
            .track_focus(&self.focus_handle)
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
                    .child(self.hours.clone()),
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

fn open_vertical_window(cx: &mut App) {
    let mut bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
    let cascade_offset = px((cx.windows().len() % 8) as f32 * 16.0);
    bounds.origin.x += cascade_offset;
    bounds.origin.y += cascade_offset;

    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            kind: WindowKind::Floating,
            titlebar: Some(TitlebarOptions {
                title: Some("PiP Vertical".into()),
                appears_transparent: true,
                traffic_light_position: Some(point(px(-200.0), px(8.0))),
            }),
            is_movable: true,
            is_resizable: true,
            is_minimizable: false,
            ..Default::default()
        },
        |window, cx| cx.new(|cx| VerticalClock::new(window, cx)),
    )
    .expect("failed to open window");
    cx.activate(true);
}

fn close_active_window(cx: &mut App) {
    let window = cx.active_window().or_else(|| {
        cx.window_stack()
            .and_then(|windows| windows.first().copied())
    });

    if let Some(window) = window {
        let _ = window.update(cx, |_, window, _| window.remove_window());
    }
}

fn main() {
    application()
        .with_quit_mode(QuitMode::LastWindowClosed)
        .run(|cx: &mut App| {
            cx.bind_keys([
                KeyBinding::new("cmd-n", NewWindow, None),
                KeyBinding::new("cmd-w", CloseWindow, None),
                KeyBinding::new("cmd-q", Quit, None),
            ]);
            cx.set_menus([Menu::new("Vertical Clock").items([
                MenuItem::action("New Vertical Clock", NewWindow),
                MenuItem::action("Close Vertical Clock", CloseWindow),
                MenuItem::separator(),
                MenuItem::action("Quit Vertical Clock", Quit),
            ])]);
            cx.on_action(|_: &NewWindow, cx| open_vertical_window(cx));
            cx.on_action(|_: &CloseWindow, cx| close_active_window(cx));
            cx.on_action(|_: &Quit, cx| cx.quit());
            open_vertical_window(cx);
        });
}
