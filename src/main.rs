mod game_engine;

use game_engine::*;

fn main() {
    let mut v1 = Vector3::new(0.0, 0.0, 0.0);
    let v2 = v1;

    println!("v1: {}", v1);
    println!("v2: {}", v2);

    let mut o = GameObject::new(v1);
    println!("pos: {}", o.get_pos());

    v1.x = 5.0;
    o.set_pos(v1);
    println!("new pos: {}", o.get_pos());

    println!("Initializing OpenGL...");
    init_gl();
    println!("OpenGL Initialized.");

    println!("Starting Game Loop...");
    start_game_loop();
    println!("Game Loop Exited.");
}
