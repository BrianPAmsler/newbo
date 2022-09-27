// Comment this out to see dead code warnings in the editor and/or debug build
#![cfg_attr(debug_assertions, allow(dead_code))]

mod game_engine;

use game_engine::*;
use game_engine::game_object::components::*;
use game_engine::game_object::GameObject;

fn main() {
    println!("Initializing Engine...");
    let mut engine = Engine::init_engine().unwrap();
    println!("Engine Initialized.");

    let root = engine.get_root_object();

    let a = GameObject::create_empty("A", Some(root.share()));
    let b = GameObject::create_empty("B", Some(root.share()));
    let c = GameObject::create_empty("C", Some(root.share()));
    let d = GameObject::create_empty("D", Some(c.share()));
    let e = GameObject::create_empty("E", Some(d.share()));
    let f = GameObject::create_empty("F", Some(c.share()));

    let comp_f = TestComponent { msg: "test f".to_owned() };
    let comp_d = TestComponent{ msg: "test d".to_owned() };

    f.add_component(Box::new(comp_f));
    d.add_component(Box::new(comp_d));

    for obj in root.get_children() {
        println!("Child: {}", obj);
    }

    a.set_parent(None);
    println!();

    for obj in root.get_children() {
        println!("Child: {}", obj);
    }
    println!();

    for obj in root.get_all_children() {
        println!("Child: {}", obj);
    }

    println!("Starting Game Loop...");
    engine.start_game_loop().unwrap();
    println!("Game Loop Exited.");
}
