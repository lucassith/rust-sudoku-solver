use crate::Selectable;
use crate::SudokuTable;
use crate::point::Point;
pub mod sequence_validator;


pub trait TableValidator {
    fn get_possibilities(&self, table: &(impl Selectable + SudokuTable), point: Vec<&Point>) -> Vec<u8>;
    fn validate(&self, table: impl Selectable + SudokuTable, point: Vec<&Point>) -> bool;
}
