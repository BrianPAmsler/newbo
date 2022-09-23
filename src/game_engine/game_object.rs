use super::Vector3;

pub struct GameObject {
    pos: Vector3
}

impl GameObject {
    pub fn create_empty() -> GameObject {
        GameObject {pos: (0.0, 0.0, 0.0).into() }
    }

    pub fn get_pos(&self) -> Vector3 {
        self.pos
    }

    pub fn set_pos(&mut self, pos: Vector3) {
        self.pos = pos;
    }
}