
use crate::{pos, Axis, Pos, Vec2};

#[derive(Clone, Copy)]
pub struct Rect {
    min: Pos,
    max: Pos 
}

impl Rect {

    pub fn new(min: Pos, max: Pos) -> Self {
        Self {
            min,
            max
        }
    }

    pub fn min_size(min: Pos, size: Vec2) -> Self {
        Self::new(min, min + size)
    }

    pub fn center_size(center: Pos, size: Vec2) -> Self {
        let min = center - size / 2.0; 
        Self::min_size(min, size)
    }

    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    pub fn min(&self) -> Pos {
        self.min
    }

    pub fn max(&self) -> Pos {
        self.max
    }

    pub fn center(&self) -> Pos {
        self.min + self.size() / 2.0 
    }

    pub fn left(&self) -> f32 {
        self.min.x
    }

    pub fn right(&self) -> f32 {
        self.max.x
    }

    pub fn top(&self) -> f32 {
        self.min.y
    }
    
    pub fn bottom(&self) -> f32 {
        self.max.y
    }

    pub fn top_left(&self) -> Pos {
        self.min
    }

    pub fn top_right(&self) -> Pos {
        pos(self.max.x, self.min.y)
    }

    pub fn bottom_left(&self) -> Pos {
        pos(self.min.x, self.max.y)
    }

    pub fn bottom_right(&self) -> Pos {
        self.max
    }

    pub fn width(&self) -> f32 {
        self.size().x
    }
    
    pub fn height(&self) -> f32 {
        self.size().y
    }

    pub fn contains(&self, pos: Pos) -> bool {
        pos.x >= self.min.x && pos.x <= self.max.x && pos.y >= self.min.y && pos.y <= self.max.y 
    }

    pub fn shift(&self, offset: Vec2) -> Self {
        Self::new(self.min + offset, self.max + offset)
    }

    pub fn dimension(&self, axis: Axis) -> f32 {
        self.size().axis(axis)
    }

}
