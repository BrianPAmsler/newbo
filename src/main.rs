// Comment this out to see dead code warnings in the editor and/or debug build
#![cfg_attr(debug_assertions, allow(dead_code))]

mod game_engine;

use game_engine::*;
use game_engine::game_object::components::*;
use game_engine::game_object::GameObject;

fn main() {
    println!("Initializing Engine...");
    let mut engine = Engine::init_engine().unwrap();
    engine.set_fixed_tick_rate(60.0);
    println!("Engine Initialized.");

    let cam = engine.get_gfx_mut().get_camera_mut();
    cam.size = Vector2::new(0.8, 0.6) * 3.0;

    let root = engine.get_root_object();

    root.borrow_mut().add_component(TestComponent::default());

    let ground = GameObject::create_empty("ground".to_owned(), Some(root.clone()));
    ground.borrow_mut().set_pos(Vector3{ x: 0.0, y: -0.75, z: 0.0 });
    let mut ground_sprite = SpriteComponent::new(0, 1);
    ground_sprite.sprite.w = 2.0;
    ground_sprite.sprite.h = 0.5;
    ground.borrow_mut().add_component(ground_sprite);
    let ground_collider = Collider::new(Polygon::new((0.0, 0.0).into(), vec![(-1.0, 0.25).into(), (1.0, 0.25).into(), (1.0, -0.25).into(), (-1.0, -0.25).into()]));
    ground.borrow_mut().add_component(ground_collider);

    let ledge = GameObject::create_empty("ground".to_owned(), Some(root.clone()));
    ledge.borrow_mut().set_pos(Vector3{ x: 0.5, y: -0.125, z: 0.0 });
    let mut ground_sprite = SpriteComponent::new(2, 1);
    ground_sprite.sprite.w = 0.5;
    ground_sprite.sprite.h = 0.75;
    ledge.borrow_mut().add_component(ground_sprite);
    let ground_collider = Collider::new(Polygon::new((0.0, 0.0).into(), vec![(-0.25, 0.375).into(), (0.25, 0.375).into(), (0.25, -0.3875).into(), (-0.25, -0.375).into()]));
    ledge.borrow_mut().add_component(ground_collider);

    let guy = GameObject::create_empty("guy".to_owned(), Some(root.clone()));
    let mut guy_sprite = SpriteComponent::new(1, 2);
    guy_sprite.sprite.w = 0.5;
    guy_sprite.sprite.h = 1.0;
    guy.borrow_mut().add_component(guy_sprite);
    guy.borrow_mut().add_component(WASDy { speed: 1.0, velocity: 0.0, acc: -5.0 });
    let guy_collider = Collider::new(Polygon::new((0.0, 0.0).into(), vec![(-0.25, 0.5).into(), (0.25, 0.5).into(), (0.25, -0.5).into(), (-0.25, -0.5).into()]));
    guy.borrow_mut().add_component(guy_collider);

    println!("Starting Game Loop...");
    engine.start_game_loop().unwrap();
    println!("Game Loop Exited.");
}
