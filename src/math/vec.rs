use std::ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axis {
    X,
    Y
}

impl Axis {

    pub fn unit(&self) -> Vec2 {
        match self {
            Axis::X => Vec2::X,
            Axis::Y => Vec2::Y,
        }
    }

    pub fn other(&self) -> Self {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::X,
        }
    }

}

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

pub const fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2 {
        x,
        y
    }
}

impl Vec2 {

    pub const ZERO: Self = Self::splat(0.0);
    pub const X: Self = vec2(1.0, 0.0);
    pub const Y: Self = vec2(0.0, 1.0);

    pub const fn splat(val: f32) -> Vec2 {
        vec2(val, val)
    }

    pub fn min(&self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x.min(other.x),
            y: self.y.min(other.y)
        }
    }

    pub fn max(&self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x.max(other.x),
            y: self.y.max(other.y)
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn axis(&self, axis: Axis) -> f32 {
        match axis {
            Axis::X => self.x,
            Axis::Y => self.y,
        }
    } 

    pub fn axis_mut(&mut self, axis: Axis) -> &mut f32 {
        match axis {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
        }
    }

}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }

}

impl AddAssign for Vec2 {

    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }

}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pos(Vec2);

pub const fn pos(x: f32, y: f32) -> Pos {
    Pos(vec2(x, y))
}

impl Pos {
    
    pub const ZERO: Self = Self::splat(0.0);

    pub const fn splat(val: f32) -> Self {
        pos(val, val)
    }

    pub fn to_vec(self) -> Vec2 {
        self.0
    }

}

impl Add<Vec2> for Pos {
    type Output = Pos;

    fn add(self, rhs: Vec2) -> Self::Output {
        Pos(self.0 + rhs)
    }

}

impl Sub<Vec2> for Pos {
    type Output = Pos;

    fn sub(self, rhs: Vec2) -> Self::Output {
        Pos(self.0 - rhs)
    }
}

impl Sub<Pos> for Pos {
    type Output = Vec2;

    fn sub(self, rhs: Pos) -> Self::Output {
        self.0 - rhs.0  
    }
}

impl Deref for Pos {
    type Target = Vec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pos {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }

}
