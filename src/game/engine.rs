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
    pub fn render_gui(&self, s: &State) {
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
        todo!();
        //split this text
        draw_text_ex(
            s.questions[0].text,
            screen_width() - 500.0,
            25.0,
            TextParams {
                font: self.constans.font,
                font_size: 25,
                color: RED,
                ..Default::default()
            },
        )
        // buttons
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
