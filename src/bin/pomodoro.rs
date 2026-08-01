use gpui::prelude::*;
use gpui::{
    App, Bounds, Context, FontWeight, QuitMode, Render, TitlebarOptions, Window, WindowBounds, WindowKind, WindowOptions, actions, div, px, rgb, size,
};
use gpui_platform::application;
use pip_clock::{tabular_figures, Theme};
use std::time::Instant;

actions!(pomo, [ToggleTheme, PlayPause, Skip, Reset]);

const WORK_SECONDS: f64 = 25.0 * 60.0;
const BREAK_SECONDS: f64 = 5.0 * 60.0;
const LONG_BREAK_SECONDS: f64 = 15.0 * 60.0;

const HEX: u32 = 0xcc3333;

const WINDOW_IS_RESIZABLE: bool = false;
const WINDOW_SIZE_X_PX: f32 = 200.0;
const WINDOW_SIZE_Y_PX: f32 = 200.0;

struct Pomodoro {
    phase: u8,
    remaining: f64,
    running: bool,
    last_tick: Option<Instant>,
    theme: Theme,
}

impl Pomodoro {
    fn new() -> Self {
        Self {
            phase: 0,
            remaining: WORK_SECONDS,
            running: false,
            last_tick: None,
            theme: Default::default(),
        }
    }

    fn is_break(&self) -> bool {
        self.phase % 2 == 1 || self.phase == 7
    }

    fn phase_name(&self) -> &'static str {
        match self.phase {
            7 => "LONG BREAK",
            p if p % 2 == 1 => "BREAK",
            _ => "WORK",
        }
    }

    fn phase_duration(phase: u8) -> f64 {
        match phase {
            7 => LONG_BREAK_SECONDS,
            p if p % 2 == 1 => BREAK_SECONDS,
            _ => WORK_SECONDS,
        }
    }

    fn advance_phase(&mut self) {
        self.phase = (self.phase + 1) % 8;
        self.remaining = Self::phase_duration(self.phase);
    }

    fn completed_sessions(&self) -> u8 {
        if self.phase >= 7 {
            4
        } else {
            self.phase / 2
        }
    }

    fn tick(&mut self) {
        if !self.running {
            return;
        }
        if let Some(last) = self.last_tick {
            let elapsed = last.elapsed().as_secs_f64();
            self.remaining -= elapsed;
            if self.remaining <= 0.0 {
                self.remaining = 0.0;
                self.running = false;
                self.advance_phase();
            }
        }
        self.last_tick = Some(Instant::now());
    }

    fn toggle_play(&mut self) {
        if self.running {
            self.running = false;
            self.last_tick = None;
        } else {
            self.running = true;
            self.last_tick = Some(Instant::now());
        }
    }

    fn skip(&mut self) {
        self.running = false;
        self.last_tick = None;
        self.advance_phase();
    }

    fn reset(&mut self) {
        self.running = false;
        self.last_tick = None;
        self.phase = 0;
        self.remaining = WORK_SECONDS;
    }
}

impl Render for Pomodoro {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.tick();
        if self.running {
            cx.notify();
        }
        window.request_animation_frame();

        let bg = self.theme.bg();
        let fg = self.theme.fg();
        let dim = self.theme.dim();
        let accent = if self.is_break() {
            rgb(HEX).into()
        } else {
            fg
        };

        let mins = (self.remaining / 60.0).floor() as u32;
        let secs = (self.remaining % 60.0).floor() as u32;
        let countdown = format!("{:02}:{:02}", mins, secs);
        let play_label = if self.running { "⏸" } else { "▶" };
        let completed = self.completed_sessions();

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .size_full()
            .bg(bg)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(accent)
                            .child(self.phase_name()),
                    )
                    .child(
                        div()
                            .mt_1()
                            .text_3xl()
                            .font_weight(FontWeight::BOLD)
                            .font_features(tabular_figures())
                            .text_color(accent)
                            .child(countdown),
                    )
                    .child(
                        div()
                            .mt_3()
                            .flex()
                            .gap_2()
                            .justify_center()
                            .children((0..4).map(|i| {
                                let color = if i < completed { accent } else { dim };
                                div().w_2().h_2().rounded_full().bg(color)
                            })),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_3()
                    .justify_center()
                    .child(
                        div()
                            .id("play")
                            .w_8()
                            .h_8()
                            .rounded_full()
                            .border_1()
                            .border_color(accent)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .text_color(accent)
                            .child(play_label)
                            .on_click(cx.listener(|this: &mut Pomodoro, _, _, cx| {
                                this.toggle_play();
                                cx.notify();
                            })),
                    )
                    .child(
                        div()
                            .id("skip")
                            .w_8()
                            .h_8()
                            .rounded_full()
                            .border_1()
                            .border_color(accent)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .text_color(accent)
                            .child("⏭")
                            .on_click(cx.listener(|this: &mut Pomodoro, _, _, cx| {
                                this.skip();
                                cx.notify();
                            })),
                    )
                    .child(
                        div()
                            .id("reset")
                            .w_8()
                            .h_8()
                            .rounded_full()
                            .border_1()
                            .border_color(accent)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_sm()
                            .text_color(accent)
                            .child("↺")
                            .on_click(cx.listener(|this: &mut Pomodoro, _, _, cx| {
                                this.reset();
                                cx.notify();
                            })),
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
                    title: Some("PiP Pomodoro".into()),
                    appears_transparent: true,
                    traffic_light_position: None,
                }),
                is_movable: true,
                is_resizable: WINDOW_IS_RESIZABLE,
                ..Default::default()
            },
            |_, cx| cx.new(|_| Pomodoro::new()),
        )
        .expect("failed to open window");
        cx.activate(true);
    });
}
