use std::ops::{Mul, Add, Sub};


/// Represents a 4x4 matrix. Indexed with standard math notation (first index is 1, not 0).
#[derive(Debug, Clone, Copy)]
pub struct Mat4x4 {
    pub values: [f32; 16]
}

impl Mat4x4 {

    /// Returns a zero matrix.
    pub fn zero() -> Mat4x4 {
        Mat4x4 { values: [0.0; 16] }
    }

    /// Returns a matrix with all ones.
    pub fn ones() -> Mat4x4 {
        Mat4x4 { values: [1.0; 16] }
    }

    /// Returns an identity matrix.
    pub fn ident() -> Mat4x4 {
        Mat4x4 { values: [ 1.0, 0.0, 0.0, 0.0,
                           0.0, 1.0, 0.0, 0.0,
                           0.0, 0.0, 1.0, 0.0,
                           0.0, 0.0, 0.0, 1.0 ]}
    }

    /// Get element a<sub>ij</sub>. Indexed with standard math notation (first index is 1, not 0).
    pub fn get(&self, i: usize, j: usize) -> f32 {
        if i < 1 || i > 4 || j < 1 || j > 4 {
            panic!("Matrix index out of range: ({}, {})", i, j);
        }
        
        self.values[(j - 1) + (i - 1)*4]
    }

    /// Set element a<sub>ij</sub>. Indexed with standard math notation (first index is 1, not 0).
    pub fn set(&mut self, i: usize, j: usize, value: f32) {
        if i < 1 || i > 4 || j < 1 || j > 4 {
            panic!("Matrix index out of range: ({}, {})", i, j);
        }
        
        self.values[(j - 1) + (i - 1)*4] = value;
    }

    /// Transpose the matrix.
    pub fn transpose(&self) -> Mat4x4 {
        let mut out = Mat4x4::zero();

        for i in 0..4 {
            for j in 0..4 {
                out.set(j, i, self.get(i, j));
            }
        }

        out
    }

    pub fn inverse(&self) -> Option<Mat4x4> {
        let m = &self.values;
        let mut inv = [0.0f32; 16];

        inv[0] = m[5]  * m[10] * m[15] - 
                m[5]  * m[11] * m[14] - 
                m[9]  * m[6]  * m[15] + 
                m[9]  * m[7]  * m[14] +
                m[13] * m[6]  * m[11] - 
                m[13] * m[7]  * m[10];

        inv[4] = -m[4]  * m[10] * m[15] + 
                m[4]  * m[11] * m[14] + 
                m[8]  * m[6]  * m[15] - 
                m[8]  * m[7]  * m[14] - 
                m[12] * m[6]  * m[11] + 
                m[12] * m[7]  * m[10];

        inv[8] = m[4]  * m[9] * m[15] - 
                m[4]  * m[11] * m[13] - 
                m[8]  * m[5] * m[15] + 
                m[8]  * m[7] * m[13] + 
                m[12] * m[5] * m[11] - 
                m[12] * m[7] * m[9];

        inv[12] = -m[4]  * m[9] * m[14] + 
                m[4]  * m[10] * m[13] +
                m[8]  * m[5] * m[14] - 
                m[8]  * m[6] * m[13] - 
                m[12] * m[5] * m[10] + 
                m[12] * m[6] * m[9];

        inv[1] = -m[1]  * m[10] * m[15] + 
                m[1]  * m[11] * m[14] + 
                m[9]  * m[2] * m[15] - 
                m[9]  * m[3] * m[14] - 
                m[13] * m[2] * m[11] + 
                m[13] * m[3] * m[10];

        inv[5] = m[0]  * m[10] * m[15] - 
                m[0]  * m[11] * m[14] - 
                m[8]  * m[2] * m[15] + 
                m[8]  * m[3] * m[14] + 
                m[12] * m[2] * m[11] - 
                m[12] * m[3] * m[10];

        inv[9] = -m[0]  * m[9] * m[15] + 
                m[0]  * m[11] * m[13] + 
                m[8]  * m[1] * m[15] - 
                m[8]  * m[3] * m[13] - 
                m[12] * m[1] * m[11] + 
                m[12] * m[3] * m[9];

        inv[13] = m[0]  * m[9] * m[14] - 
                m[0]  * m[10] * m[13] - 
                m[8]  * m[1] * m[14] + 
                m[8]  * m[2] * m[13] + 
                m[12] * m[1] * m[10] - 
                m[12] * m[2] * m[9];

        inv[2] = m[1]  * m[6] * m[15] - 
                m[1]  * m[7] * m[14] - 
                m[5]  * m[2] * m[15] + 
                m[5]  * m[3] * m[14] + 
                m[13] * m[2] * m[7] - 
                m[13] * m[3] * m[6];

        inv[6] = -m[0]  * m[6] * m[15] + 
                m[0]  * m[7] * m[14] + 
                m[4]  * m[2] * m[15] - 
                m[4]  * m[3] * m[14] - 
                m[12] * m[2] * m[7] + 
                m[12] * m[3] * m[6];

        inv[10] = m[0]  * m[5] * m[15] - 
                m[0]  * m[7] * m[13] - 
                m[4]  * m[1] * m[15] + 
                m[4]  * m[3] * m[13] + 
                m[12] * m[1] * m[7] - 
                m[12] * m[3] * m[5];

        inv[14] = -m[0]  * m[5] * m[14] + 
                m[0]  * m[6] * m[13] + 
                m[4]  * m[1] * m[14] - 
                m[4]  * m[2] * m[13] - 
                m[12] * m[1] * m[6] + 
                m[12] * m[2] * m[5];

        inv[3] = -m[1] * m[6] * m[11] + 
                m[1] * m[7] * m[10] + 
                m[5] * m[2] * m[11] - 
                m[5] * m[3] * m[10] - 
                m[9] * m[2] * m[7] + 
                m[9] * m[3] * m[6];

        inv[7] = m[0] * m[6] * m[11] - 
                m[0] * m[7] * m[10] - 
                m[4] * m[2] * m[11] + 
                m[4] * m[3] * m[10] + 
                m[8] * m[2] * m[7] - 
                m[8] * m[3] * m[6];

        inv[11] = -m[0] * m[5] * m[11] + 
                m[0] * m[7] * m[9] + 
                m[4] * m[1] * m[11] - 
                m[4] * m[3] * m[9] - 
                m[8] * m[1] * m[7] + 
                m[8] * m[3] * m[5];

        inv[15] = m[0] * m[5] * m[10] - 
                m[0] * m[6] * m[9] - 
                m[4] * m[1] * m[10] + 
                m[4] * m[2] * m[9] + 
                m[8] * m[1] * m[6] - 
                m[8] * m[2] * m[5];

        let mut det = m[0] * inv[0] + m[1] * inv[4] + m[2] * inv[8] + m[3] * inv[12];

        if det == 0.0 {
            return None;
        }

        det = 1.0 / det;

        for i in 0..16 {
            inv[i] *= det;
        }

        Some(Mat4x4 { values: inv })
    }
}

impl<'a, 'b> Mul<&'b Mat4x4> for &'a Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, rhs: &'b Mat4x4) -> Self::Output {
        let a = self;
        let b = rhs;

        let mut c = Mat4x4::zero();

        // The compiler probably could have unrolled this for me, but whatever.
        c.set(1, 1, a.get(1, 1) * b.get(1, 1) + a.get(1, 2) * b.get(2, 1) + a.get(1, 3) * b.get(3, 1) + a.get(1, 4) * b.get(4, 1));
        c.set(1, 2, a.get(1, 1) * b.get(1, 2) + a.get(1, 2) * b.get(2, 2) + a.get(1, 3) * b.get(3, 2) + a.get(1, 4) * b.get(4, 2));
        c.set(1, 3, a.get(1, 1) * b.get(1, 3) + a.get(1, 2) * b.get(2, 3) + a.get(1, 3) * b.get(3, 3) + a.get(1, 4) * b.get(4, 3));
        c.set(1, 4, a.get(1, 1) * b.get(1, 4) + a.get(1, 2) * b.get(2, 4) + a.get(1, 3) * b.get(3, 4) + a.get(1, 4) * b.get(4, 4));
        c.set(2, 1, a.get(2, 1) * b.get(1, 1) + a.get(2, 2) * b.get(2, 1) + a.get(2, 3) * b.get(3, 1) + a.get(2, 4) * b.get(4, 1));
        c.set(2, 2, a.get(2, 1) * b.get(1, 2) + a.get(2, 2) * b.get(2, 2) + a.get(2, 3) * b.get(3, 2) + a.get(2, 4) * b.get(4, 2));
        c.set(2, 3, a.get(2, 1) * b.get(1, 3) + a.get(2, 2) * b.get(2, 3) + a.get(2, 3) * b.get(3, 3) + a.get(2, 4) * b.get(4, 3));
        c.set(2, 4, a.get(2, 1) * b.get(1, 4) + a.get(2, 2) * b.get(2, 4) + a.get(2, 3) * b.get(3, 4) + a.get(2, 4) * b.get(4, 4));
        c.set(3, 1, a.get(3, 1) * b.get(1, 1) + a.get(3, 2) * b.get(2, 1) + a.get(3, 3) * b.get(3, 1) + a.get(3, 4) * b.get(4, 1));
        c.set(3, 2, a.get(3, 1) * b.get(1, 2) + a.get(3, 2) * b.get(2, 2) + a.get(3, 3) * b.get(3, 2) + a.get(3, 4) * b.get(4, 2));
        c.set(3, 3, a.get(3, 1) * b.get(1, 3) + a.get(3, 2) * b.get(2, 3) + a.get(3, 3) * b.get(3, 3) + a.get(3, 4) * b.get(4, 3));
        c.set(3, 4, a.get(3, 1) * b.get(1, 4) + a.get(3, 2) * b.get(2, 4) + a.get(3, 3) * b.get(3, 4) + a.get(3, 4) * b.get(4, 4));
        c.set(4, 1, a.get(4, 1) * b.get(1, 1) + a.get(4, 2) * b.get(2, 1) + a.get(4, 3) * b.get(3, 1) + a.get(4, 4) * b.get(4, 1));
        c.set(4, 2, a.get(4, 1) * b.get(1, 2) + a.get(4, 2) * b.get(2, 2) + a.get(4, 3) * b.get(3, 2) + a.get(4, 4) * b.get(4, 2));
        c.set(4, 3, a.get(4, 1) * b.get(1, 3) + a.get(4, 2) * b.get(2, 3) + a.get(4, 3) * b.get(3, 3) + a.get(4, 4) * b.get(4, 3));
        c.set(4, 4, a.get(4, 1) * b.get(1, 4) + a.get(4, 2) * b.get(2, 4) + a.get(4, 3) * b.get(3, 4) + a.get(4, 4) * b.get(4, 4));
        
        c
    }
}

impl Mul for Mat4x4 {
    type Output = Mat4x4;
    
    fn mul(self, rhs: Mat4x4) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<Mat4x4> for f32 {
    type Output = Mat4x4;

    fn mul(self, mut rhs: Mat4x4) -> Self::Output {
        for element in &mut rhs.values {
            *element *= self;
        }

        rhs
    }
}

impl Mul<f32> for Mat4x4 {
    type Output = Mat4x4;

    fn mul(self, rhs: f32) -> Self::Output {
        rhs * self
    }
}

impl<'a, 'b> Add<&'b Mat4x4> for &'a Mat4x4 {
    type Output = Mat4x4;

    fn add(self, rhs: &'b Mat4x4) -> Self::Output {
        let mut out = Mat4x4::zero();

        for i in 0..16 {
            out.values[i] = self.values[i] + rhs.values[i];
        }

        out
    }

}

impl Add for Mat4x4 {
    type Output = Mat4x4;

    fn add(mut self, rhs: Self) -> Self::Output {
        for i in 0..16 {
            self.values[i] += rhs.values[i];
        }

        self
    }
}

impl<'a, 'b> Sub<&'b Mat4x4> for &'a Mat4x4 {
    type Output = Mat4x4;

    fn sub(self, rhs: &'b Mat4x4) -> Self::Output {
        let mut out = Mat4x4::zero();

        for i in 0..16 {
            out.values[i] = self.values[i] - rhs.values[i];
        }

        out
    }

}

impl Sub for Mat4x4 {
    type Output = Mat4x4;

    fn sub(mut self, rhs: Self) -> Self::Output {
        for i in 0..16 {
            self.values[i] -= rhs.values[i];
        }

        self
    }
}