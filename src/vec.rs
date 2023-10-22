use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, Default)]
#[repr(C)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Float3 {
    pub fn distance_square(&self, another: Float3) -> f32 {
        self.x * another.x + self.y * another.y + self.z * another.z + 0.000001
    }
    pub fn normalize(&self) -> Float3 {
        let distance = self.distance_square(*self).sqrt();
        *self / distance
    }
}
impl Add<Float3> for Float3 {
    type Output = Float3;

    fn add(self, rhs: Float3) -> Float3 {
        Float3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Float3> for Float3 {
    type Output = Float3;

    fn sub(self, rhs: Float3) -> Float3 {
        Float3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul<f32> for Float3 {
    type Output = Float3;

    fn mul(self, rhs: f32) -> Float3 {
        Float3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Float3> for f32 {
    type Output = Float3;

    fn mul(self, rhs: Float3) -> Float3 {
        Float3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<f32> for Float3 {
    type Output = Float3;

    fn div(self, rhs: f32) -> Float3 {
        Float3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl AddAssign<Float3> for Float3 {
    fn add_assign(&mut self, rhs: Float3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
