use macroquad::prelude::*;

use super::sprite::Sprite;

pub struct Engine {
    pub sprites: Vec<Sprite>,
    pub constans: Constants,
}
pub struct Constants {
    font: Font,
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
            r.render();
            if !r.text.is_empty() {
                r.render_text(&self.constans.font);
            }
        }
    }
    pub fn render_gui(&self) {
        draw_rectangle(
            screen_width() - 200.0,
            0.0,
            screen_width(),
            screen_height(),
            YELLOW,
        );
    }
}

impl Constants {
    async fn new() -> Self {
        Self {
            font: load_ttf_font("Comfortaa-Regular.ttf").await.unwrap(),
        }
    }
}
