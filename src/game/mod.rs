mod engine;
mod questions;
mod sprite;
mod state;
mod strings;

use egui::style::Margin;
use egui::{FontFamily::*, Frame};
use egui::{FontId, TextStyle::*};
use engine::Engine;
use macroquad::prelude::*;
use questions::Questions;

use sprite::Sprite;
use state::State;

#[derive(PartialEq, Eq)]
pub enum Closed {
    Nope,
    Requested,
    Yep,
}

pub struct Game {
    fps: i32,
    engine: Engine,
    state: State,
    closed: Closed,
    questions: Questions,
    user_state: UserState,
}

pub enum UserState {
    MainMenu,
    AboutDialog,
    WaitingQuestion,
    Restart,
    QuestionDialog,
    HintDialog(usize),
    SuccessEndDialog,
    FailEndDialog,
}

impl Game {
    pub async fn new() -> Game {
        Self {
            fps: 0,
            engine: Engine::new(vec![
                Sprite::new("Вася", "resources/images/gigaman.png", true, (0.0, 300.0)).await,
            ])
            .await,
            state: State::new(),
            closed: Closed::Nope,
            questions: Questions::new(),
            user_state: UserState::MainMenu,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed == Closed::Yep
    }

    pub fn reqwest_close(&mut self) {
        self.closed = Closed::Requested
    }

    pub async fn logick(&mut self) {
        if is_key_down(KeyCode::Up) {
            self.state.add_hp(5);
        }
        if is_key_down(KeyCode::Down) {
            self.state.remove_hp(5);
        }
        if is_key_down(KeyCode::Escape) {
            self.closed = Closed::Requested;
        }

        if let UserState::Restart = self.user_state {
            self.questions = Questions::new();
            self.state = State::new();
            self.user_state = UserState::WaitingQuestion;
        }
        if let UserState::WaitingQuestion = self.user_state {
            match self.questions.get_random_question() {
                Ok(_) => self.user_state = UserState::QuestionDialog,
                Err(_) => self.user_state = UserState::SuccessEndDialog,
            }
        }
        if self.state.is_dead() {
            self.questions = Questions::new();
            self.state = State::new();
            self.user_state = UserState::FailEndDialog;
        }
    }

    pub async fn render(&mut self) {
        self.fps = get_fps();

        // clear background
        clear_background(WHITE);

        egui_macroquad::ui(|ctx| {
            let mut style = (*ctx.style()).clone();

            // custom font
            style.text_styles = [
                (Heading, FontId::new(45.0, Proportional)),
                (Body, FontId::new(30.0, Proportional)),
                (Button, FontId::new(30.0, Proportional)),
                (Small, FontId::new(10.0, Proportional)),
            ]
            .into();
            style.visuals.window_shadow.extrusion = 0.;

            // custom style for frames(windows)
            self.engine.constants.frame_style =
                Frame::window(&style).inner_margin(Margin::same(20.));

            // apply style
            ctx.set_style(style);

            // render scenes
            match self.user_state {
                UserState::MainMenu => {
                    self.engine
                        .render_main(ctx, &mut self.closed, &mut self.user_state)
                }
                UserState::WaitingQuestion => (),
                UserState::QuestionDialog => {
                    self.engine.render_question(
                        ctx,
                        &mut self.user_state,
                        &mut self.state,
                        self.questions.current.as_ref(),
                    );
                    self.engine.render_stats(ctx, &self.state);
                }
                UserState::HintDialog(_) => {
                    self.engine.render_hint(
                        ctx,
                        &mut self.user_state,
                        self.questions.current.as_ref(),
                    );
                    self.engine.render_stats(ctx, &self.state);
                }
                UserState::SuccessEndDialog => {
                    self.engine.render_success(ctx, &mut self.user_state)
                }
                UserState::FailEndDialog => self.engine.render_failure(ctx, &mut self.user_state),
                UserState::Restart => (),
                UserState::AboutDialog => self.engine.render_about(ctx, &mut self.user_state),
            }
            if self.closed == Closed::Requested {
                self.closed = self.engine.render_exit_dialog(ctx);
            }
        });

        // draw fps in debug mode
        if cfg!(debug_assertions) {
            draw_text(&format!("{}", self.fps), 10.0, 20.0, 36.0, BLACK);
        }

        // draw egui before next frame
        egui_macroquad::draw();
        next_frame().await;
    }
}
