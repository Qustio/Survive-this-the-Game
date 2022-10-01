mod engine;
mod sprite;
mod state;
mod strings;

use engine::Engine;
use macroquad::prelude::*;
use sprite::Sprite;
use state::State;

#[derive(PartialEq)]
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
}

impl Game {
    pub async fn new() -> Self {
        Self {
            fps: 0,
            engine: Engine::new(vec![
                Sprite::new("Вася", "resources/images/gigaman.png", true, (0.0, 300.0)).await,
            ])
            .await,
            state: State::new(),
            closed: Closed::Nope,
        }
    }

    pub fn is_closed(&self) -> bool {
        self.closed == Closed::Yep
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
            self.closed = Closed::Requested;
        }
    }

    pub async fn render(&mut self) {
        self.fps = get_fps();
        // Clear background
        clear_background(WHITE);
        // draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        // draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        // draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_text("IT WORKS!", 20.0, 20.0, 30.0, DARKGRAY);
        self.engine.render();
        self.engine.render_gui();
        if self.closed == Closed::Requested {
            self.closed = self.engine.render_exit_dialog();
        }
        // Draw fps in debug mode
        if cfg!(debug_assertions) {
            draw_text(&format!("{}", self.fps), 10.0, 20.0, 36.0, BLACK);
        }
        next_frame().await;
    }
}
