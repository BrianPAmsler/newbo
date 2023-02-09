// Comment this out to see dead code warnings in the editor and/or debug build
#![cfg_attr(debug_assertions, allow(dead_code))]

mod game_engine;

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

    let ground = GameObject::create_empty("ground".to_owned(), Some(root.share()));
    ground.set_pos(Vector3{ x: 0.0, y: -0.75, z: 0.0 });
    let mut ground_sprite = SpriteComponent::new(0, 1);
    ground_sprite.sprite.w = 2.0;
    ground_sprite.sprite.h = 0.5;
    ground.add_component(Box::new(ground_sprite));
    let ground_collider = Collider::new(2.0, 0.5, None);
    ground.add_component(Box::new(ground_collider));

    let guy = GameObject::create_empty("guy".to_owned(), Some(root.share()));
    let mut guy_sprite = SpriteComponent::new(1, 2);
    guy_sprite.sprite.w = 0.5;
    guy_sprite.sprite.h = 1.0;
    guy.add_component(Box::new(guy_sprite));
    guy.add_component(Box::new(WASDy { speed: 1.0 }));
    let guy_collider = Collider::new(0.5, 1.0, Some(Box::new(|_, _| (println!("collide!")))));
    guy.add_component(Box::new(guy_collider));

    println!("Starting Game Loop...");
    engine.start_game_loop().unwrap();
    println!("Game Loop Exited.");
}
