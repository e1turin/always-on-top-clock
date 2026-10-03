use gpui::prelude::*;
use gpui::{
    actions, div, point, px, size, App, Bounds, Context, FocusHandle, FontWeight, KeyBinding, Menu,
    MenuItem, QuitMode, Render, SharedString, TitlebarOptions, Window, WindowBounds, WindowKind,
    WindowOptions,
};
use gpui_platform::application;
use pip_clock::{tabular_figures, Theme};
use std::time::{Duration, Instant};

actions!(clock, [ToggleTheme, NewWindow, CloseWindow]);

struct Clock {
    focus_handle: FocusHandle,
    time_text: SharedString,
    last_update: Instant,
    theme: Theme,
}

impl Clock {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);

        Self {
            focus_handle,
            time_text: Self::current_time().into(),
            last_update: Instant::now(),
            theme: Default::default(),
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
            .on_action(|_: &CloseWindow, window, _| window.remove_window())
            .track_focus(&self.focus_handle)
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

fn open_clock_window(cx: &mut App) {
    let mut bounds = Bounds::centered(None, size(px(300.0), px(300.0)), cx);
    let cascade_offset = px((cx.windows().len() % 8) as f32 * 16.0);
    bounds.origin.x += cascade_offset;
    bounds.origin.y += cascade_offset;

    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            kind: WindowKind::Floating,
            titlebar: Some(TitlebarOptions {
                title: Some("PiP Clock".into()),
                appears_transparent: true,
                traffic_light_position: Some(point(px(-200.0), px(8.0))),
            }),
            is_movable: true,
            is_resizable: true,
            is_minimizable: false,
            ..Default::default()
        },
        |window, cx| cx.new(|cx| Clock::new(window, cx)),
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
            ]);
            cx.set_menus([Menu::new("Clock").items([
                MenuItem::action("New Clock", NewWindow),
                MenuItem::action("Close Clock", CloseWindow),
            ])]);
            cx.on_action(|_: &NewWindow, cx| open_clock_window(cx));
            cx.on_action(|_: &CloseWindow, cx| close_active_window(cx));
            open_clock_window(cx);
        });
}
