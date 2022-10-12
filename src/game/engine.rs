use macroquad::{
    hash,
    prelude::*,
    ui::{root_ui, Skin},
};

use super::{sprite::Sprite, state::State, Closed};

pub struct Engine {
    pub sprites: Vec<Sprite>,
    pub constans: Constants,
}

pub struct Constants {
    font: Font,
    ui_skin: Skin,
}

impl Engine {
    pub async fn new(sprites: Vec<Sprite>) -> Self {
        Self {
            sprites,
            constans: Constants::new().await,
        }
    }
    pub fn render(&self) {
        for r in &self.sprites {
            if !r.hidden {
                r.render();
                if !r.text.is_empty() {
                    r.render_text(&self.constans.font);
                }
            }
        }
    }
    pub fn render_gui(&self, s: &mut State) {
        // left pane
        draw_rectangle(
            screen_width() - 500.0,
            0.0,
            screen_width(),
            screen_height(),
            GRAY,
        );
        draw_rectangle_lines(
            screen_width() - 500.0,
            0.0,
            screen_width(),
            screen_height(),
            2.0,
            BLACK,
        );
        // stats
        draw_text_ex(
            &format!("День: {}/{}", s.day.0, s.day.1),
            10.,
            100.,
            TextParams {
                font: self.constans.font,
                font_size: 25,
                color: BLACK,
                ..Default::default()
            },
        );
        draw_text_ex(
            &format!("Очки здоровья: {}/{}", s.hp.0, s.hp.1),
            10.,
            130.,
            TextParams {
                font: self.constans.font,
                font_size: 25,
                color: BLACK,
                ..Default::default()
            },
        );
        draw_text_ex(
            &format!("Насыщение: {}/{}", s.saturation.0, s.saturation.1),
            10.,
            160.,
            TextParams {
                font: self.constans.font,
                font_size: 25,
                color: BLACK,
                ..Default::default()
            },
        );
        draw_text_ex(
            &format!("Жажда: {}/{}", s.water.0, s.water.1),
            10.,
            190.,
            TextParams {
                font: self.constans.font,
                font_size: 25,
                color: BLACK,
                ..Default::default()
            },
        );
        //split this text
        let mut row = 25.;
        for t in s.questions[0].text.split('\n') {
            draw_text_ex(
                t,
                screen_width() - 450.,
                row + 30.,
                TextParams {
                    font: self.constans.font,
                    font_size: 25,
                    color: BLACK,
                    ..Default::default()
                },
            );
            row += 25.0;
        }
        // buttons
        let a1 = s.questions[0].answers.0.to_owned();
        let a2 = s.questions[0].answers.1.to_owned();
        root_ui().push_skin(&self.constans.ui_skin);
        root_ui().window(
            hash!(),
            vec2(1200., 700.),
            vec2(200., 100.),
            |ui: &mut macroquad::ui::Ui| {
                if ui.button(None, a1) {
                    s.questions[0].result.0(s);
                }
                ui.same_line(120.);
                if ui.button(None, a2) {
                    s.questions[0].result.1(s);
                }
            },
        );
        root_ui().pop_skin();
    }

    pub fn render_exit_dialog(&self) -> Closed {
        let dialog_size = vec2(400., 150.);
        let screen_size = vec2(screen_width(), screen_height());
        let dialog_position = screen_size / 2. - dialog_size / 2.;

        let mut exit = Closed::Requested;
        root_ui().push_skin(&self.constans.ui_skin);
        root_ui().window(
            hash!(),
            dialog_position,
            dialog_size,
            |ui: &mut macroquad::ui::Ui| {
                ui.label(None, "Вы действительно хотите выйти?");
                ui.separator();
                ui.same_line(60.);
                if ui.button(None, "Да") {
                    exit = Closed::Yep;
                }
                ui.same_line(120.);
                if ui.button(None, "Нет") {
                    exit = Closed::Nope;
                }
            },
        );
        root_ui().pop_skin();
        exit
    }
}

impl Constants {
    async fn new() -> Self {
        let skin = {
            let label_style = root_ui()
                .style_builder()
                .font(include_bytes!(
                    "../../resources/fonts/Comfortaa-Regular.ttf"
                ))
                .unwrap()
                .text_color(BLACK)
                .font_size(14)
                .build();
            let _window_style = root_ui()
                .style_builder()
                // .background(Image::from_file_with_format(
                //     include_bytes!("../examples/ui_assets/window_background.png"),
                //     None,
                // ))
                //.background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
                //.margin(RectOffset::new(0.0, 0.0, 0.0, 0.0))
                .color_inactive(RED)
                .color_hovered(GREEN)
                .color_clicked(BLUE)
                .reverse_background_z(true)
                .build();
            let button_style = root_ui()
                .style_builder()
                // .background(Image::from_file_with_format(
                //     include_bytes!("../examples/ui_assets/button_background.png"),
                //     None,
                // ))
                // .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
                .margin(RectOffset::new(0.0, 0.0, 10.0, 10.0))
                //.background_margin(RectOffset::new(0.0, 0.0, 10.0, 10.0))
                //.background_hovered(Image::from_file_with_format(
                //    include_bytes!("../examples/ui_assets/button_hovered_background.png"),
                //    None,
                // ))
                // .background_clicked(Image::from_file_with_format(
                //     include_bytes!("../examples/ui_assets/button_clicked_background.png"),
                //     None,
                // ))
                .font(include_bytes!(
                    "../../resources/fonts/Comfortaa-Regular.ttf"
                ))
                .unwrap()
                .color(GRAY)
                .text_color(BLACK)
                .font_size(20)
                .build();
            //     let editbox_style = root_ui()
            //         .style_builder()
            //         .background_margin(RectOffset::new(0., 0., 0., 0.))
            //         .font(include_bytes!(
            //             "../../resources/fonts/Comfortaa-Regular.ttf"
            //         ))
            //         .unwrap()
            //         .text_color(BLACK)
            //         .color_selected(RED)
            //         .font_size(10)
            //         .build();

            Skin {
                label_style,
                button_style,
                ..root_ui().default_skin()
            }
        };
        Self {
            font: load_ttf_font("resources/fonts/Comfortaa-Regular.ttf")
                .await
                .unwrap(),
            ui_skin: skin,
        }
    }
}
