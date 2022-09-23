// Comment this out to see dead code warnings in the editor and/or debug build
#![cfg_attr(debug_assertions, allow(dead_code))]

mod game_engine;

use game_engine::*;

fn main() {
    let mut v1 = Vector3::new(0.0, 0.0, 0.0);
    let v2 = v1;

    println!("v1: {}", v1);
    println!("v2: {}", v2);

    let mut o = GameObject::create_empty("Test", None);
    println!("pos: {}", o.get_pos());

    v1.x = 5.0;
    o.set_pos(v1);
    println!("new pos: {}", o.get_pos());

    println!("Initializing OpenGL...");
    let mut engine = Engine::init_engine().unwrap();
    println!("OpenGL Initialized.");

    println!("Starting Game Loop...");
    engine.start_game_loop().unwrap();
    println!("Game Loop Exited.");
}
