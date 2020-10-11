use std::fmt::Debug;
use std::cmp::PartialEq;
use std::clone::Clone;

#[derive(Debug, PartialEq)]
pub enum Axis {
    X, 
    Y
}
#[derive(Debug, PartialEq)]
pub enum CoordinateError {
    OutOfRange(Axis)
}


#[derive(Debug, Clone)]
pub struct Point { pub x: usize, pub y: usize }

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}