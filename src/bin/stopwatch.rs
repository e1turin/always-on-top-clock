use gpui::prelude::*;
use gpui::{
    actions, div, point, px, size, App, Bounds, Context, FocusHandle, FontWeight, KeyBinding, Menu,
    MenuItem, MouseButton, QuitMode, Render, TitlebarOptions, Window, WindowBounds, WindowKind,
    WindowOptions,
};
use gpui_platform::application;
use pip_clock::{tabular_figures, Theme};
use std::time::{Duration, Instant};

actions!(
    stopwatch_actions,
    [ToggleTheme, Stop, Reset, NewWindow, CloseWindow, Quit]
);

const WINDOW_IS_RESIZABLE: bool = false;
const WINDOW_SIZE_X_PX: f32 = 150.0;
const WINDOW_SIZE_Y_PX: f32 = 150.0;

struct Stopwatch {
    focus_handle: FocusHandle,
    /// Time accumulated while stopped.
    accumulated: Duration,
    /// When the current running stretch started.
    started_at: Option<Instant>,
    running: bool,
    intervals: Vec<Duration>,
    showing_intervals: bool,
    theme: Theme,
}

impl Stopwatch {
    fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let focus_handle = cx.focus_handle();
        focus_handle.focus(window, cx);

        Self {
            focus_handle,
            accumulated: Duration::ZERO,
            started_at: None,
            running: false,
            intervals: Vec::new(),
            showing_intervals: false,
            theme: Default::default(),
        }
    }

    fn elapsed(&self) -> Duration {
        let running_elapsed = self
            .started_at
            .map_or(Duration::ZERO, |started_at| started_at.elapsed());
        self.accumulated + running_elapsed
    }

    fn toggle_stop(&mut self) {
        if self.running {
            self.accumulated = self.elapsed();
            self.started_at = None;
            self.running = false;
        } else {
            self.started_at = Some(Instant::now());
            self.running = true;
        }
    }

    fn reset(&mut self) {
        self.intervals.push(self.elapsed());
        self.accumulated = Duration::ZERO;
        if self.running {
            self.started_at = Some(Instant::now());
        }
    }

    fn format_duration(duration: Duration) -> String {
        let hours = duration.as_secs() / 3600;
        let minutes = (duration.as_secs() / 60) % 60;
        let seconds = duration.as_secs() % 60;
        format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
    }
}

impl Render for Stopwatch {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.running {
            cx.notify();
        }
        window.request_animation_frame();

        let bg = self.theme.bg();
        let fg = self.theme.fg();
        let muted = self.theme.muted();
        let dim = self.theme.dim();
        let accent = fg;

        if self.showing_intervals {
            let intervals = self
                .intervals
                .iter()
                .enumerate()
                .map(|(index, duration)| {
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .w_full()
                        .px_3()
                        .py_px()
                        .text_sm()
                        .font_features(tabular_figures())
                        .text_color(fg)
                        .child(format!("{}.", index + 1))
                        .child(Self::format_duration(*duration))
                })
                .collect::<Vec<_>>();

            div()
                .on_action(|_: &CloseWindow, window, _| window.remove_window())
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.activate(true))
                .track_focus(&self.focus_handle)
                .relative()
                .size_full()
                .bg(bg)
                .child(
                    div()
                        .id("intervals")
                        .flex()
                        .flex_col()
                        .size_full()
                        .pt_8()
                        .pb_2()
                        .overflow_y_scroll()
                        .when(intervals.is_empty(), |this| {
                            this.items_center()
                                .justify_center()
                                .child(div().text_sm().text_color(muted).child("No intervals"))
                        })
                        .children(intervals),
                )
                .child(
                    div()
                        .id("back")
                        .absolute()
                        .top_1()
                        .left_1()
                        .w_6()
                        .h_6()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_lg()
                        .rounded_full()
                        .bg(dim)
                        .shadow_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(accent)
                        .child("←")
                        .on_click(cx.listener(|this: &mut Stopwatch, _, _, cx| {
                            this.showing_intervals = false;
                            cx.notify();
                        })),
                )
        } else {
            let time_text = Self::format_duration(self.elapsed());
            let stop_label = if self.running { " ⏸" } else { "▶" };
            let interval_count_text = self.intervals.len().to_string();

            div()
                .on_action(|_: &CloseWindow, window, _| window.remove_window())
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.activate(true))
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
                        .child(time_text),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_4()
                        .child(
                            div()
                                .id("count")
                                .w_6()
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_sm()
                                .font_features(tabular_figures())
                                .text_color(muted)
                                .child(interval_count_text)
                                .on_click(cx.listener(|this: &mut Stopwatch, _, _, cx| {
                                    this.showing_intervals = true;
                                    cx.notify();
                                })),
                        )
                        .child(
                            div()
                                .id("stop")
                                .px_2()
                                .rounded_full()
                                .border_dashed()
                                .flex()
                                .text_sm()
                                .text_color(accent)
                                .font_weight(FontWeight::BOLD)
                                .child(stop_label)
                                .on_click(cx.listener(|this: &mut Stopwatch, _, _, cx| {
                                    this.toggle_stop();
                                    cx.notify();
                                })),
                        )
                        .child(
                            div()
                                .id("reset")
                                .px_2()
                                .rounded_full()
                                .border_dashed()
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_sm()
                                .text_color(accent)
                                .font_weight(FontWeight::BOLD)
                                .child("↺")
                                .on_click(cx.listener(|this: &mut Stopwatch, _, _, cx| {
                                    this.reset();
                                    cx.notify();
                                })),
                        ),
                )
        }
    }
}

fn open_stopwatch_window(cx: &mut App) {
    let mut bounds = Bounds::centered(None, size(px(WINDOW_SIZE_X_PX), px(WINDOW_SIZE_Y_PX)), cx);
    let cascade_offset = px((cx.windows().len() % 8) as f32 * 16.0);
    bounds.origin.x += cascade_offset;
    bounds.origin.y += cascade_offset;

    cx.open_window(
        WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            // PopUp windows stay visible when the application loses focus. The
            // root element activates the app when clicked so shortcuts still work.
            kind: WindowKind::PopUp,
            titlebar: Some(TitlebarOptions {
                title: Some("PiP Stopwatch".into()),
                appears_transparent: true,
                traffic_light_position: Some(point(px(-200.0), px(8.0))),
            }),
            is_movable: true,
            is_resizable: WINDOW_IS_RESIZABLE,
            is_minimizable: false,
            ..Default::default()
        },
        |window, cx| cx.new(|cx| Stopwatch::new(window, cx)),
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
            cx.set_menus([Menu::new("Stopwatch").items([
                MenuItem::action("New Stopwatch", NewWindow),
                MenuItem::action("Close Stopwatch", CloseWindow),
                MenuItem::separator(),
                MenuItem::action("Quit Stopwatch", Quit),
            ])]);
            cx.on_action(|_: &NewWindow, cx| open_stopwatch_window(cx));
            cx.on_action(|_: &CloseWindow, cx| close_active_window(cx));
            cx.on_action(|_: &Quit, cx| cx.quit());
            open_stopwatch_window(cx);
        });
}
