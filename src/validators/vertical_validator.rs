use super::TableValidator;
use crate::point::Point;
use crate::Selectable;
use std::collections::HashMap;


pub struct VerticalValidator {
    
}

impl<T> TableValidator<T> for VerticalValidator where 
    T: Selectable {
    fn validate(self, table: &T) -> bool {
        return true;
    }

    fn get_possibilites(self, table: &T, point: &Point) -> Vec<u8> {
        let mut current_values = vec!(0u8; T::SIZE);
        let mut possible_values = vec!(0u8; T::SIZE);
        for i in 0..T::SIZE {
            possible_values[i] = (i + 1) as u8; 
            current_values[i] = table.value_in_point(&Point{x: point.x, y: i}).unwrap();
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
        table = *table.set_in_point(&Point{x: 2, y: 0}, 4).unwrap();

        let possibilites = validator.get_possibilites(&table, &Point{x: 2, y: 6});
        assert_eq!(possibilites.len(), 6);
        assert_eq!(possibilites.contains(&1), true);
        assert_eq!(possibilites.contains(&2), false);
        assert_eq!(possibilites.contains(&3), true);
        assert_eq!(possibilites.contains(&4), false);
        assert_eq!(possibilites.contains(&5), true);
        assert_eq!(possibilites.contains(&6), false);
        assert_eq!(possibilites.contains(&7), true);
        assert_eq!(possibilites.contains(&8), true);
        assert_eq!(possibilites.contains(&8), true);
    }
}