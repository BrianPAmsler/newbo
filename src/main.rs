mod game_engine;

fn main() {
    println!("Initializing OpenGL...");
    game_engine::init_gl();
    println!("OpenGL Initialized.");

    println!("Starting Game Loop...");
    game_engine::start_game_loop();
    println!("Game Loop Exited.");
}
