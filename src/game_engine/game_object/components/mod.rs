mod test_component;
pub use test_component::TestComponent;

use super::GameObject;

pub trait Component {
    fn update(&mut self, _delta_time: f64, _owner: &GameObject) {}
    fn fixed_update(&mut self, _delta_time: f64, _owner: &GameObject) {}
    fn render(&mut self, _delta_time: f64, _owner: &GameObject) {}
}