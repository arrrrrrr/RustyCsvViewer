use serde::{Deserialize,Serialize};

#[derive(Debug,Default,Deserialize,Serialize,PartialEq)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}