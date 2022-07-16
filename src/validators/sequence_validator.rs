use crate::Selectable;
use crate::SudokuTable;
use super::TableValidator;
use crate::point::Point;

pub struct SequenceValidator {

}

impl TableValidator for SequenceValidator {
    fn get_possibilities(&self, table: &(impl Selectable + SudokuTable), point: Vec<&Point>) -> Vec<u8> {
        let possibilities = table.possible_values();
        let current_values: Vec<u8> = point.into_iter().filter_map(|p| -> Option<u8> {
            let value_in_point = table.value_in_point(p).unwrap();
            if value_in_point == 0 {
                return Option::None;
            }
            Some(value_in_point)
        }).collect();
        possibilities.into_iter().filter(|v| !current_values.iter().any(|c| c == v)).collect()
    }

    fn validate(&self, _table: impl Selectable + SudokuTable, _point: Vec<&Point>) -> bool {
        true
    }
}

#[cfg(test)]
mod test {
    use crate::table::Table;
    use super::SequenceValidator;
    use super::TableValidator;
    use crate::Settable;
    use crate::point::Point;
    #[test]
    fn test_should_return_valid_possibilities() {
        let mut t = Table::new();
        t = *t.set_in_point(&Point{x: 1, y: 2}, 5).unwrap();
        t = *t.set_in_point(&Point{x: 1, y: 3}, 6).unwrap();

        let s = SequenceValidator{};
        let mut point_vec = Vec::new();
        point_vec.push(&Point{x: 1, y: 2});
        point_vec.push(&Point{x: 1, y: 3});
        point_vec.push(&Point{x: 1, y: 6});
        let possible_vales = s.get_possibilities(&t, point_vec);
        assert_eq!(possible_vales.len(), 7);
        assert_eq!(possible_vales[0], 1);
        assert_eq!(possible_vales[1], 2);
        assert_eq!(possible_vales[2], 3);
        assert_eq!(possible_vales[3], 4);
        assert_eq!(possible_vales[4], 7);
        assert_eq!(possible_vales[5], 8);
        assert_eq!(possible_vales[6], 9);
    }
}
