mod test_component;
pub use test_component::TestComponent;

#[allow(unused_variables)]
pub trait Component {
    fn update(&self, delta_time: f32) {}

    fn render(&self, delta_time: f32) {}
}