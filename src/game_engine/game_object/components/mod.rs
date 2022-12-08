mod test_component;

use downcast_rs::{Downcast, impl_downcast};
pub use test_component::TestComponent;

use crate::game_engine::Engine;

use super::GameObject;

pub struct TickInfo<'a> {
    pub(in crate::game_engine) delta_time: f64,
    pub(in crate::game_engine) engine: &'a Engine
}

pub trait Component: Downcast {
    fn init(&mut self, _engine: &Engine, _owner: &GameObject) {}
    fn update(&mut self, _info: TickInfo, _owner: &GameObject) {}
    fn fixed_update(&mut self, _info: TickInfo, _owner: &GameObject) {}
    fn render(&mut self, _info: TickInfo, _owner: &GameObject) {}
}

impl_downcast!(Component);