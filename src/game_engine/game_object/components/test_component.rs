use super::Component;

pub struct TestComponent {
    pub msg: String
}

impl Component for TestComponent {
    fn update(&self, delta_time: f32) {
        println!("Message: {}", self.msg);
    }
}