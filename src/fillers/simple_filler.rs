use super::Filler;
use crate::point::Point;
use crate::{SudokuTable, Selectable, Settable, SquareTable};
use crate::validators::sequence_validator::SequenceValidator;
use crate::validators::TableValidator;
use crate::point_selection::line_point_selection::{SelectionType, LinePointSelection};

pub struct SimpleFiller {}

impl Filler for SimpleFiller {
    fn fill<T>(&self, table: &T, point: &Point) -> Option<Vec<T>>
        where T: Selectable + Settable + SudokuTable + SquareTable {
        let sequence_validator = SequenceValidator{};
        let line_point_selection = LinePointSelection{};
        let zone_points = table.points_in_zone(point).unwrap();
        let zone_possibilites = sequence_validator.get_possibilities(table, zone_points.iter().collect());
        let vertical_points = line_point_selection.get_points(table, point, SelectionType::Vertical).unwrap();
        let horizontal_points = line_point_selection.get_points(table, point, SelectionType::Horizontal).unwrap();
        let vertical_possibilites = sequence_validator.get_possibilities(table, vertical_points.iter().collect());
        let horizontal_possibilites = sequence_validator.get_possibilities(table, horizontal_points.iter().collect());
        let common_possibilites = zone_possibilites.iter()
            .filter(|v| vertical_possibilites.iter().any(|c| c == *v))
            .filter(|v| horizontal_possibilites.iter().any(|c| c == *v))
            .collect::<Vec<&u8>>();

        if common_possibilites.is_empty() {
            return None
        }
        let mut table_vec = Vec::<T>::with_capacity(common_possibilites.len());
        for v in common_possibilites {
            table_vec.push(*table.set_in_point(point, *v).unwrap());
        }
        Some(table_vec)
    }
}
