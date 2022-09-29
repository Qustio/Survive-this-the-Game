use macroquad::prelude::*;

pub struct Sprite {
    pub text: String,
    pub texture: Texture2D,
    pub hidden: bool,
    pub pos: (f32, f32),
}

impl Sprite {
    pub async fn new(text: &str, texture_path: &str, hidden: bool, pos: (f32, f32)) -> Self {
        Self {
            text: text.to_string(),
            texture: load_texture(texture_path).await.unwrap(),
            hidden,
            pos,
        }
    }
}

impl Sprite {
    pub fn render(&self) {
        draw_texture(self.texture, self.pos.0, self.pos.1, WHITE);
    }
    pub fn render_text(&self, font: &Font) {
        let text_params = TextParams {
            font: *font,
            ..Default::default()
        };
        draw_text_ex(
            &self.text.to_string(),
            self.pos.0 + self.texture.width() / 2.0 - 100.0,
            self.pos.1 + self.texture.height() - 50.0,
            text_params,
        );
    }
}
