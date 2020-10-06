pub mod zone_validator;
pub mod horizontal_validator;
pub mod vertical_validator;
use crate::Selectable;
use crate::point::Point;

pub trait TableValidator<T: Selectable> {
    fn get_possibilites(self, table: &T, point: dyn Iterator<Item = Point>) -> Vec<u8>;
    fn validate(self, table: &T) -> bool;
}