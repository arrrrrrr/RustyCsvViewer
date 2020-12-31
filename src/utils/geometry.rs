use serde::{Deserialize,Serialize};

#[derive(Debug,Default,Deserialize,Serialize,PartialEq)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug,Default,PartialEq)]
pub struct Rect<T>
{
    x: T,
    y: T,
    width: T,
    height: T
}

impl<T> Rect<T> {
    pub fn set_width(&mut self, width: T) {
        self.width = width
    }

    pub fn set_height(&mut self, height: T) {
        self.height = height
    }

    pub fn set_x(&mut self, x: T) {
        self.x = x
    }

    pub fn set_y(&mut self, y: T) {
        self.y = y
    }

    pub fn x(&self) -> &T {
        &self.x
    }

    pub fn y(&self) -> &T {
        &self.y
    }

    pub fn width(&self) -> &T {
        &self.width
    }

    pub fn height(&self) -> &T {
        &self.height
    }
}