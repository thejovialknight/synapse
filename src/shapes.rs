use crate::vec2::Vec2;

pub struct Circle {
    pub origin: Vec2,
    pub radius: f32
}

pub struct Line {
    pub start: Vec2,
    pub end: Vec2
}

impl Line {
    pub fn translate(&self, other: &Vec2) -> Line {
        return Line{
            start: Vec2{x: self.start.x + other.x, y: self.start.y + other.y},
            end: Vec2{x: self.end.x + other.x, y: self.end.y + other.y}
        };
    }
}
