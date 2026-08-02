use gpui::prelude::*;
use gpui::{
    App, Bounds, Context, FontWeight, QuitMode, Render, TitlebarOptions, Window, WindowBounds, WindowKind, WindowOptions, actions, div, px, size,
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
    /// How many times the timer has been reset.
    reset_count: u32,
    theme: Theme,
}

impl Timer {
    fn new() -> Self {
        Self {
            accumulated: Duration::ZERO,
            started_at: None,
            running: false,
            reset_count: 0,
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
        self.accumulated = Duration::ZERO;
        self.reset_count += 1;
        if self.running {
            self.started_at = Some(Instant::now());
        }
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

        let elapsed = self.elapsed();
        let hours = elapsed.as_secs() / 3600;
        let minutes = (elapsed.as_secs() / 60) % 60;
        let seconds = elapsed.as_secs() % 60;
        let time_text = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
        let stop_label = if self.running { " ⏸" } else { "▶" };
        let reset_counter_text = format!("{}", self.reset_count);

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
                    )
                    .child(
                        div()
                            .w_6()
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .font_features(tabular_figures())
                            .text_color(muted)
                            .child(reset_counter_text),
                    ),
            )
    }
}

fn main() {
    application().with_quit_mode(QuitMode::LastWindowClosed).run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(WINDOW_SIZE_X_PX), px(WINDOW_SIZE_Y_PX)), cx);

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
