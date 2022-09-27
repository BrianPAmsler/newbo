use std::{rc::Rc, cell::RefCell};

use super::Vector3;

#[allow(unused_variables)]
pub trait Component {
    fn update(&self, delta_time: f32) {}

    fn render(&self, delta_time: f32) {}

    fn share(&self) -> Box<dyn Component>;
}

struct _GameObject {
    name: String,
    pos: Vector3,
    components: Vec<Box<dyn Component>>,
    children: Vec<GameObject>,
    parent: Option<GameObject>
}

pub struct GameObject {
    obj: Rc<RefCell<_GameObject>>
}

impl GameObject {
    pub fn share(&self) -> GameObject {
        GameObject { obj: self.obj.clone() }
    }
}

impl std::fmt::Display for GameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameObject{{\"{}\"}}", self.obj.as_ref().borrow().name)
    }
}

impl AsRef<GameObject> for GameObject {
    fn as_ref(&self) -> &GameObject {
        self
    }
}

impl GameObject {
    pub fn create_empty(name: &str, parent: Option<GameObject>) -> GameObject {
        let mut newobj = GameObject { obj: Rc::new(RefCell::new(_GameObject {
            name: name.to_owned(),
            pos: (0.0, 0.0, 0.0).into(),
            components: Vec::new(),
            children: Vec::new(),
            parent: None 
        }))};
        
        newobj.set_parent(parent);

        newobj
    }

    pub fn get_pos(&self) -> Vector3 {
        self.obj.as_ref().borrow().pos
    }

    pub fn set_pos(&mut self, pos: Vector3) {
        self.obj.borrow_mut().pos = pos;
    }

    pub fn update(&self, delta_time: f32) {
        for cmp in &self.obj.as_ref().borrow().components[..] {
            cmp.update(delta_time);
        }
    }

    pub fn render(&self, delta_time: f32) {
        for cmp in &self.obj.as_ref().borrow().components[..] {
            cmp.render(delta_time);
        }
    }

    pub fn add_component(&mut self, component: &dyn Component) {
        self.obj.borrow_mut().components.push(component.share());
    }

    pub fn get_parent(&self) -> Option<GameObject> {
        match &self.obj.as_ref().borrow().parent {
            Some(parent) => Some(parent.share()),
            None => None
        }
    }

    pub fn set_parent(&mut self, parent: Option<GameObject>) {
        {
            let old_parent = &self.obj.as_ref().borrow().parent;

            match old_parent {
                Some(p) => p.share().remove_child(self),
                None => ()
            }
        }

        match parent {
            Some(mut p) => {
                p.add_child(self.share());
                self.obj.borrow_mut().parent = Some(p);
            },
            None => self.obj.borrow_mut().parent = None
        }
    }

    pub fn get_childeren(&self) -> Vec<GameObject> {
        let mut v = Vec::new();
        v.reserve(self.obj.as_ref().borrow().children.len());

        for c in &self.obj.as_ref().borrow().children {
            v.push(c.share());
        }

        v
    }

    fn add_child(&mut self, child: GameObject) {
        self.obj.borrow_mut().children.push(child.share());
    }

    fn remove_child(&mut self, child: &GameObject) {
        let mut idx = -1;
        for (i, c) in self.obj.borrow_mut().children.iter().enumerate() {
            if Rc::ptr_eq(&c.obj, &child.obj) {
                idx = i as i32;
                break;
            }
        }

        if idx == 1 {
            panic!("Child does not exist! This shouldn't happen!");
        }

        self.obj.borrow_mut().children.remove(idx as usize);
    }
}