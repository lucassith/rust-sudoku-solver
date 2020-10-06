use super::TableValidator;
use crate::point::Point;
use crate::Selectable;
use crate::Zonable;

pub struct VerticalValidator {
    
}

impl<T> TableValidator<T> for VerticalValidator where 
    T: Selectable + Zonable {
    fn validate(self, table: &T) -> bool {
        return true;
    }

    fn get_possibilites(self, table: &T, point: &Point) -> Vec<u8> {
        let mut current_values = vec!(0u8; T::SIZE);
        let mut possible_values = vec!(0u8; T::SIZE);
        for i in 0..T::SIZE {
            possible_values[i] = (i + 1) as u8; 
        }
        for point in table.points_in_zone(point).unwrap() {
            current_values.push(table.value_in_point(&point).unwrap());
        }
        return possible_values.into_iter().filter(|v| !current_values.iter().any(|c| c == v)).collect();
    }
}

#[cfg(test)]
mod test {
    use super::VerticalValidator;
    use crate::validators::TableValidator;
    use crate::*;
    use crate::table::Table;
    use crate::point::Point;
    #[test]
    fn test_should_return_valid_possibilites() {
        let mut table = Table::new();
        let validator = VerticalValidator{};
        
        table = *table.set_in_point(&Point{x: 2, y: 5}, 2).unwrap();
        table = *table.set_in_point(&Point{x: 2, y: 7}, 6).unwrap();
        table = *table.set_in_point(&Point{x: 3, y: 3}, 5).unwrap();
        table = *table.set_in_point(&Point{x: 4, y: 5}, 1).unwrap();
        table = *table.set_in_point(&Point{x: 4, y: 4}, 4).unwrap();

        let possible_value = validator.get_possibilites(&table, &Point{x: 4, y: 3});
        assert_eq!(possible_value.len(), 6);
        assert_eq!(possible_value.contains(&1), false);
        assert_eq!(possible_value.contains(&2), true);
        assert_eq!(possible_value.contains(&3), true);
        assert_eq!(possible_value.contains(&4), false);
        assert_eq!(possible_value.contains(&5), false);
        assert_eq!(possible_value.contains(&6), true);
        assert_eq!(possible_value.contains(&7), true);
        assert_eq!(possible_value.contains(&8), true);
        assert_eq!(possible_value.contains(&9), true);

    }
}