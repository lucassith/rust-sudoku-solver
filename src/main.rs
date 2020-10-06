mod validators;
pub mod table;
pub mod point;
use point::Point;
use point::CoordinateError;
use validators::TableValidator;

pub trait Selectable {
    const SIZE: usize;
    fn value_in_point(&self, point: &Point) -> Result<u8, CoordinateError>;
}

pub trait Settable {
    fn set_in_point(&self, point: &Point, value: u8) -> Result<Box<Self>, CoordinateError>;
}

pub trait Zonable {
    fn points_in_zone(&self, point: &Point) -> Result<Vec<Point>, CoordinateError>;
}


fn main() {
    
    
}



