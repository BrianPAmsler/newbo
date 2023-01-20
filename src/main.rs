// Comment this out to see dead code warnings in the editor and/or debug build
#![cfg_attr(debug_assertions, allow(dead_code))]
#![feature(cell_filter_map)]

mod game_engine;

use std::fs::File;
use std::io::BufReader;

use game_engine::*;
use game_engine::game_object::components::*;
use game_engine::game_object::GameObject;

extern crate rand;

struct Test {
    pub n: i32
}

fn main() {
    println!("Initializing Engine...");
    let mut engine = Engine::init_engine().unwrap();
    engine.set_fixed_tick_rate(60.0);
    println!("Engine Initialized.");

    let root = engine.get_root_object();

    root.add_component(Box::new(TestComponent::default()));

    for i in 0..500 {
        let x = rand::random::<f32>() * 2.0 - 1.0;
        let y = rand::random::<f32>() * 2.0 - 1.0;
        let w = rand::random::<f32>() * 0.1 + 0.01;
        let h = rand::random::<f32>() * 0.1 + 0.01;

        let id = rand::random::<u32>() % 16 + 1;

        let obj = GameObject::create_empty(format!("obj #{}", i), Some(root.share()));
        let mut sprite = SpriteComponent::new(i, id as i32);
        sprite.sprite = Sprite { x, y, w, h, sprite_id: id as i32};

        obj.add_component(Box::new(sprite));
        obj.add_component(Box::new(WASDy));
    }

    println!("Starting Game Loop...");
    engine.start_game_loop().unwrap();
    println!("Game Loop Exited.");
}
