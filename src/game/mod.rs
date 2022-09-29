mod engine;
mod sprite;
mod state;

use engine::Engine;
use macroquad::prelude::*;
use sprite::Sprite;
use state::State;

pub struct Game {
    fps: i32,
    engine: Engine,
    state: State,
    closed: bool,
}

impl Game {
    pub async fn new() -> Self {
        Self {
            fps: 0,
            engine: Engine::new(vec![
                Sprite::new("Вася", "gigaman.png", true, (0.0, 300.0)).await,
            ])
            .await,
            state: State::new(),
            closed: false,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed
    }

    pub async fn logick(&mut self) {
        // todo!();
        if is_key_down(KeyCode::Up) && self.state.hp <= 250 {
            self.state.hp += 5;
        }
        if is_key_down(KeyCode::Down) && self.state.hp >= 5 {
            self.state.hp -= 5;
        }
        if is_key_down(KeyCode::Escape) {
            self.closed = true;
        }
    }

    pub async fn render(&mut self) {
        clear_background(RED);
        self.fps = get_fps();
        draw_text(&format!("{}", self.fps), 100.0, 100.0, 36.0, WHITE);
        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        self.engine.render();
        self.engine.render_gui();
        next_frame().await;
    }
}
