pub mod simple_filler;
use crate::{SudokuTable, Selectable, Settable, SquareTable};
use crate::point::Point;


pub trait Filler {
    fn fill<T>(&self, t: &T, p: &Point) -> Option<Vec<T>> where T: SudokuTable + Settable + Selectable + SquareTable;
}