pub trait Cartesian {
    fn get_x(&self) -> f32;
    fn get_y(&self) -> f32;
}

enum Node<T: Cartesian> {
    Empty,
    Object(T),
    Node(Box<Node<T>>, Box<Node<T>>, Box<Node<T>>, Box<Node<T>>)
}

pub struct QuadTree<T>
  where T: Cartesian {
        
    root: Node<T>
}

impl<T: Cartesian> QuadTree<T> {
    fn new() -> QuadTree<T> {
        QuadTree { root: Node::Empty }
    }
}