use gpui::prelude::*;
use gpui::{
    actions, div, px, size, App, Bounds, Context, FontWeight, QuitMode, Render, TitlebarOptions,
    Window, WindowBounds, WindowKind, WindowOptions,
};
use gpui_platform::application;
use pip_clock::{tabular_figures, Theme};
use std::time::{Duration, Instant};

actions!(timer_actions, [ToggleTheme, Stop, Reset]);

const WINDOW_IS_RESIZABLE: bool = false;
const WINDOW_SIZE_X_PX: f32 = 150.0;
const WINDOW_SIZE_Y_PX: f32 = 150.0;

struct Timer {
    /// Time accumulated while stopped.
    accumulated: Duration,
    /// When the current running stretch started.
    started_at: Option<Instant>,
    running: bool,
    intervals: Vec<Duration>,
    showing_intervals: bool,
    theme: Theme,
}

impl Timer {
    fn new() -> Self {
        Self {
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

impl Render for Timer {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.running {
            cx.notify();
        }
        window.request_animation_frame();

        let bg = self.theme.bg();
        let fg = self.theme.fg();
        let muted = self.theme.muted();
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
                        .py_1()
                        .text_sm()
                        .font_features(tabular_figures())
                        .text_color(fg)
                        .child(format!("{}.", index + 1))
                        .child(Self::format_duration(*duration))
                })
                .collect::<Vec<_>>();

            div()
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
                        .left_2()
                        .w_6()
                        .h_6()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_lg()
                        .font_weight(FontWeight::BOLD)
                        .text_color(accent)
                        .child("<")
                        .on_click(cx.listener(|this: &mut Timer, _, _, cx| {
                            this.showing_intervals = false;
                            cx.notify();
                        })),
                )
        } else {
            let time_text = Self::format_duration(self.elapsed());
            let stop_label = if self.running { " ⏸" } else { "▶" };
            let interval_count_text = self.intervals.len().to_string();

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
                                .on_click(cx.listener(|this: &mut Timer, _, _, cx| {
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
                                .on_click(cx.listener(|this: &mut Timer, _, _, cx| {
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
                                .on_click(cx.listener(|this: &mut Timer, _, _, cx| {
                                    this.reset();
                                    cx.notify();
                                })),
                        ),
                )
        }
    }
}

fn main() {
    application()
        .with_quit_mode(QuitMode::LastWindowClosed)
        .run(|cx: &mut App| {
            let bounds =
                Bounds::centered(None, size(px(WINDOW_SIZE_X_PX), px(WINDOW_SIZE_Y_PX)), cx);

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    kind: WindowKind::PopUp,
                    titlebar: Some(TitlebarOptions {
                        title: Some("PiP Timer".into()),
                        appears_transparent: true,
                        traffic_light_position: None,
                    }),
                    is_movable: true,
                    is_resizable: WINDOW_IS_RESIZABLE,
                    ..Default::default()
                },
                |_, cx| cx.new(|_| Timer::new()),
            )
            .expect("failed to open window");
            cx.activate(true);
        });
}
