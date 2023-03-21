use super::Vector2;

#[derive(Clone)]
pub struct Polygon {
    pub center: Vector2,
    verticies: Vec<Vector2>
}

impl Polygon {
    pub fn new(center: Vector2, verticies: Vec<Vector2>) -> Polygon {
        Polygon { center, verticies }
    }

    pub fn get_edges(&self) -> Vec<(Vector2, Vector2)> {
        let mut out = Vec::new();

        for i in 0..self.verticies.len() {
            let a = self.verticies[i] + self.center;
            let b = self.verticies[(i + 1) % self.verticies.len()] + self.center;

            out.push((a, b));
        }

        out
    }

    pub fn get_points(&self) -> Vec<Vector2> {
        self.verticies.iter().map(|x| x.to_owned() + self.center).collect()
    }

    pub fn collide(&self, other: &Polygon) -> Option<Vector2> {
        let mut push = None;
        let mut mag = f32::MAX;
        for edge in self.get_edges() {
            let axis = (edge.1 - edge.0).ortho().normalize();

            let v = match get_push_vector(self, other, axis) {
                Some(v) => v,
                None => return None
            };

            let m = v.dot(v);
            if m < mag {
                push = Some(v);
                mag = m;
            }
        }

        push
    }
}

fn get_shadow(shape: &Polygon, line: Vector2) -> (f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;

    for point in shape.get_points() {
        let dot = line.dot(point);

        min = min.min(dot);
        max = max.max(dot);
    }

    (min, max)
}

fn get_push_vector(p1: &Polygon, p2: &Polygon, axis: Vector2) -> Option<Vector2> {
    let s1 = get_shadow(p1, axis);
    let s2 = get_shadow(p2, axis);

    let push_scalar = if s1.0 < s2.0 {
        if s1.1 < s2.0 {
            return None;
        }

        s2.0 - s1.1
    } else {
        if s2.1 < s1.0 {
            return None;
        }

        s2.1 - s1.0
    };

    Some(axis * push_scalar)
}