use std::ops::Neg;

#[derive(Copy, Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        return Vec2{x: -self.x, y: -self.y};
    }
}

impl Vec2 {
    pub fn normalize(&self) -> Vec2 {
        // Divide self by magnitude
        return self.divide_by_f32((self.x.powf(2.0) + self.y.powf(2.0)).sqrt());
    }

    pub fn add(&self, other: &Vec2) -> Vec2 {
        return Vec2{x: self.x + other.x, y: self.y + other.y};
    }

    pub fn add_by_f32(&self, other: f32) -> Vec2 {
        return Vec2{x: self.x + other, y: self.y + other};
    }

    pub fn subtract(&self, other: &Vec2) -> Vec2 {
        return Vec2{x: self.x - other.x, y: self.y - other.y};
    }

    pub fn subtract_by_f32(&self, other: f32) -> Vec2 {
        return Vec2{x: self.x - other, y: self.y - other};
    }

    pub fn multiply(&self, other: &Vec2) -> Vec2 {
        return Vec2{x: self.x * other.x, y: self.y * other.y};
    }

    pub fn multiply_by_f32(&self, other: f32) -> Vec2 {
        return Vec2{x: self.x * other, y: self.y * other};
    }

    pub fn divide(&self, other: &Vec2) -> Vec2 {
        return Vec2{x: self.x / other.x, y: self.y / other.y};
    }

    pub fn divide_by_f32(&self, other: f32) -> Vec2 {
        return Vec2{x: self.x / other, y: self.y / other};
    }
}
