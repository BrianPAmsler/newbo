// Comment this out to see dead code warnings in the editor and/or debug build
#![cfg_attr(debug_assertions, allow(dead_code))]

mod game_engine;

use game_engine::*;

fn main() {
    let root = GameObject::create_empty("Root Object", None);

    let mut a = GameObject::create_empty("A", Some(root.share()));
    let b = GameObject::create_empty("B", Some(root.share()));
    let c = GameObject::create_empty("C", Some(root.share()));

    for obj in root.get_childeren() {
        println!("Child: {}", obj);
    }

    a.set_parent(None);
    println!();

    for obj in root.get_childeren() {
        println!("Child: {}", obj);
    }

    println!("Initializing OpenGL...");
    let mut engine = Engine::init_engine().unwrap();
    println!("OpenGL Initialized.");

    println!("Starting Game Loop...");
    engine.start_game_loop().unwrap();
    println!("Game Loop Exited.");
}
