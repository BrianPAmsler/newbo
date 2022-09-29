// Comment this out to see dead code warnings in the editor and/or debug build
#![cfg_attr(debug_assertions, allow(dead_code))]

mod game_engine;

use game_engine::*;
use game_engine::game_object::components::*;
use game_engine::game_object::GameObject;

extern crate rand;

fn main() {
    println!("Initializing Engine...");
    let mut engine = Engine::init_engine().unwrap();
    println!("Engine Initialized.");

    let root = engine.get_root_object();

    let n = 1000;

    let mut objs = Vec::new();
    objs.push(root.share());
    objs.reserve(n);

    for i in 0..n {
        let choice = rand::random::<usize>() % objs.len();

        let s = format!("test {}", i);

        objs.push(GameObject::create_empty(&s[..], Some(objs[choice].share())));

        if i == n - 1 {
            objs[i].add_component(Box::new(TestComponent::default()));
        }
    }

    println!("Starting Game Loop...");
    engine.start_game_loop().unwrap();
    println!("Game Loop Exited.");
}
