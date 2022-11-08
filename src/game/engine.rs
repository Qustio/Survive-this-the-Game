use std::{collections::HashMap, hash::Hash};

use egui::Context;
use egui::{style::Margin, Frame};
use egui_extras::RetainedImage;
use macroquad::prelude::*;

use super::{questions::Question, sprite::Sprite, state::State, Closed, UserState};

pub struct Engine {
    pub sprites: Vec<Sprite>,
    pub constants: Constants,
}

pub struct Constants {
    pub font: Font,
    pub frame_style: Frame,
    pub sprites: HashMap<String, Texture2D>,
    pub textures: HashMap<String, RetainedImage>,
}

impl Engine {
    pub async fn new(sprites: Vec<Sprite>) -> Self {
        Self {
            sprites,
            constants: Constants::new().await,
        }
    }

    pub fn render_main(&self, ctx: &egui::Context, closed: &mut Closed, us: &mut UserState) {
        draw_texture(self.constants.sprites["main"], 0., 0., WHITE);
        egui::Window::new("menu")
            .frame(self.constants.frame_style)
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Выживай-ка");
                });
                ui.separator();
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Начать").clicked() {
                        *us = UserState::WaitingQuestion;
                    }
                    if ui.button("Разработчики").clicked() {
                        *us = UserState::AboutDialog;
                    }
                    if ui.button("Выход").clicked() {
                        *closed = Closed::Requested;
                    }
                });
            });
    }

    pub fn render_stats(&self, ctx: &egui::Context, s: &State) {
        egui::Window::new("stats")
            .frame(self.constants.frame_style)
            .title_bar(false)
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(30.0, 20.0))
            .auto_sized()
            .show(ctx, |ui| {
                ui.label(&format!("День: {}/{}", s.day(), s.max_day()));
                ui.label(&format!("Очки здоровья: {}/{}", s.hp(), s.max_hp()));
                ui.label(&format!("Голод: {}/{}", s.saturation(), s.max_saturation()));
                ui.label(&format!("Жажда: {}/{}", s.water(), s.max_water()));
            });
    }

    pub fn render_success(&self, ctx: &egui::Context, us: &mut UserState) {
        egui::Window::new("menu")
            .frame(self.constants.frame_style)
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Вам удалось выжить!");
                });
                ui.separator();
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Начать заново").clicked() {
                        *us = UserState::Restart;
                    }
                    if ui.button("Главное меню").clicked() {
                        *us = UserState::MainMenu;
                    }
                });
            });
    }

    pub fn render_failure(&self, ctx: &egui::Context, us: &mut UserState) {
        egui::Window::new("menu")
            .frame(self.constants.frame_style)
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.vertical_centered_justified(|ui| {
                    ui.heading("Вы погибли!");
                });
                ui.separator();
                ui.vertical_centered_justified(|ui| {
                    if ui.button("Начать заново").clicked() {
                        *us = UserState::Restart;
                    }
                    if ui.button("Главное меню").clicked() {
                        *us = UserState::MainMenu;
                    }
                });
            });
    }

    pub fn render_question(
        &self,
        ctx: &egui::Context,
        us: &mut UserState,
        state: &mut State,
        q: Option<&Question>,
    ) {
        if let Some(q) = q {
            egui::Window::new("questiontextw")
                .title_bar(false)
                .anchor(egui::Align2::RIGHT_CENTER, egui::Vec2::ZERO)
                .fixed_size(egui::Vec2::new(500., 900.))
                .fixed_pos(egui::pos2(1100., 0.))
                .show(ctx, |ui| {
                    ui.set_min_height(500.);
                    egui::TopBottomPanel::top("top_panel")
                        .resizable(false)
                        .frame(
                            Frame::default()
                                .inner_margin(Margin::same(20.))
                                .outer_margin(Margin::same(5.)),
                        )
                        .min_height(32.0)
                        .show_inside(ui, |ui| {
                            ui.label(q.text);
                        });
                    egui::TopBottomPanel::bottom("bot_panel")
                        .resizable(false)
                        .frame(Frame::default().inner_margin(Margin::same(20.)))
                        .min_height(32.0)
                        .show_inside(ui, |ui| {
                            ui.vertical_centered_justified(|ui| {
                                ui.heading("Выберите дейтвие");
                            });
                            ui.separator();
                            ui.vertical_centered_justified(|ui| {
                                for (i, a) in q.answers.answers.iter().enumerate() {
                                    if ui.button(a.get_answer()).clicked() {
                                        a.make_choice(state);
                                        *us = UserState::HintDialog(i);
                                    }
                                }
                            });
                        });
                });
        }
    }

    pub fn render_hint(&self, ctx: &egui::Context, us: &mut UserState, q: Option<&Question>) {
        if let UserState::HintDialog(hi) = us {
            if let Some(q) = q {
                let hint = q.answers.answers[*hi].get_hint();
                egui::Window::new("questiontextw")
                    .title_bar(false)
                    .enabled(false)
                    .anchor(egui::Align2::RIGHT_CENTER, egui::Vec2::ZERO)
                    .fixed_size(egui::Vec2::new(500., 900.))
                    .fixed_pos(egui::pos2(1100., 0.))
                    .show(ctx, |ui| {
                        ui.set_min_height(500.);
                        egui::TopBottomPanel::top("top_panel")
                            .resizable(false)
                            .frame(
                                Frame::default()
                                    .inner_margin(Margin::same(20.))
                                    .outer_margin(Margin::same(5.)),
                            )
                            .min_height(32.0)
                            .show_inside(ui, |ui| {
                                ui.label(q.text);
                            });
                        egui::TopBottomPanel::bottom("bot_panel")
                            .resizable(false)
                            .frame(Frame::default().inner_margin(Margin::same(20.)))
                            .min_height(32.0)
                            .show_inside(ui, |ui| {
                                ui.vertical_centered_justified(|ui| {
                                    ui.heading("Выберите дейтвие");
                                });
                                ui.separator();
                                ui.vertical_centered_justified(|ui| {
                                    for (_i, a) in q.answers.answers.iter().enumerate() {
                                        if ui.button(a.get_answer()).clicked() {}
                                    }
                                });
                            });
                    });

                egui::Window::new("hintw")
                    .title_bar(false)
                    .resizable(false)
                    .frame(self.constants.frame_style)
                    .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0., 0.))
                    .show(ctx, |ui| {
                        ui.label(hint);
                        ui.vertical_centered(|ui| {
                            if ui.button("Ok").clicked() {
                                *us = UserState::WaitingQuestion;
                            }
                        });
                    });
            }
        }
    }

    pub fn render_exit_dialog(&self, ctx: &egui::Context) -> Closed {
        let mut exit = Closed::Requested;
        egui::Window::new("hintw")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.set_min_width(430.);
                ui.label("Вы действительно хотите выйти?");
                ui.separator();
                ui.columns(2, |columns| {
                    if columns[0].button("Да").clicked() {
                        exit = Closed::Yep;
                    }
                    if columns[1].button("Нет").clicked() {
                        exit = Closed::Nope;
                    }
                });
            });
        exit
    }

    pub fn render_about(&self, ctx: &egui::Context, us: &mut UserState) {
        egui::Window::new("hintw")
            .title_bar(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.heading("Авторы");
                ui.separator();
                ui.label("Гура Павел");
                ui.label("Маслихов Иван");
                ui.vertical_centered(|ui| {
                    if ui.button("Назад").clicked() {
                        *us = UserState::MainMenu;
                    }
                });
            });
    }
}

impl Constants {
    async fn new() -> Self {
        let mut sprites = HashMap::new();
        sprites.insert(
            "main".to_string(),
            load_texture("resources/images/main.png").await.unwrap(),
        );
        let mut textures = HashMap::new();
        Self {
            font: load_ttf_font("resources/fonts/Comfortaa-Regular.ttf")
                .await
                .unwrap(),
            frame_style: Frame::default(),
            sprites,
            textures,
        }
    }
}
