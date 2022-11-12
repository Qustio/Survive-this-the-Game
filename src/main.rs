#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use game::Game;

mod game;

fn window_conf() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "Выживай-ка".to_owned(),
        fullscreen: true,
        window_resizable: true,
        window_width: 1600,
        window_height: 900,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;
    while !game.is_closed() {
        game.logick().await;
        game.render().await;
    }
}
