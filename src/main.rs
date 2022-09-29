use game::Game;

mod game;
#[macroquad::main("Выживай-ка")]
async fn main() {
    let mut game = Game::new().await;
    while !game.is_closed() {
        game.logick().await;
        game.render().await;
    }
}
