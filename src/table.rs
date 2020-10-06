use crate::point::Point;
use crate::point::Axis;
use crate::point::CoordinateError;
use std::fmt;

pub const TABLE_SIZE: usize = 9;

pub struct Table {
    fields: [[u8; TABLE_SIZE]; TABLE_SIZE]
}

impl Table {
    pub fn new() -> Table {
        return Table {
            fields: [[0u8; TABLE_SIZE]; TABLE_SIZE]
        }
    }

    fn checkRange(&self, point: &Point) -> Result<(), CoordinateError> {
        if point.x > 8 {
            return Result::Err(CoordinateError::OutOfRange(Axis::X))
        }
        if point.y > 8 {
            return Result::Err(CoordinateError::OutOfRange(Axis::Y))
        }
        return Result::Ok(());
    }
}

impl crate::Zonable for Table {
    fn points_in_zone(&self, point: &Point) -> Result<Vec<Point>, CoordinateError> {
        match self.checkRange(&point) {
            Result::Err(x) => return Result::Err(x),
            _ => (),
        }
        let x_offset = (point.x / 3) * 3;
        let y_offset = (point.y / 3) * 3;
        let mut points = Vec::with_capacity(9);
        for x in 0..3 {
            for y in 0..3 {
                points.push(Point{x: x_offset + x, y: y_offset + y})
            }
        }
        return Result::Ok(points);
    }
}

impl fmt::Debug for Table {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut row: &[u8; 9];
        writeln!(f, "┌───────┬───────┬───────┐").ok();
        let val = |v: u8| -> String {
            if v == 0 {
                return String::from(" ");
            }
            return v.to_string();
        };
        for i in 0..9 {
            row = &self.fields[i];
            writeln!(
                f, 
                "│ {} {} {} │ {} {} {} │ {} {} {} │", 
                val(row[0]), val(row[1]), val(row[2]), val(row[3]), val(row[4]), val(row[5]), val(row[6]), val(row[7]), val(row[8])
            ).ok();
            if (i + 1) % 3 == 0 && i != 8 {
                writeln!(f, "├───────┼───────┼───────┤").ok();
            }
        }
        writeln!(f, "└───────┴───────┴───────┘").ok();
        return Result::Ok(());
    }
}

impl super::Settable for Table {
    fn set_in_point(&self, point: &Point, value: u8) -> Result<Box<Self>, CoordinateError> {
        match self.checkRange(&point) {
            Result::Err(x) => return Result::Err(x),
            _ => (),
        }

        let mut fields = self.fields.clone();
        fields[point.x][point.y] = value;
        return Result::Ok(Box::from(Table{fields: fields}));
    }
}

impl super::Selectable for Table {
    const SIZE: usize = TABLE_SIZE;
    fn value_in_point(&self, point: &Point) -> Result<u8, CoordinateError> {
        match self.checkRange(&point) {
            Result::Err(x) => return Result::Err(x),
            _ => (),
        }
        return Result::Ok(self.fields[point.x][point.y]);
    }


}

#[cfg(test)]
mod test {
    use super::Table;
    use crate::point::Point;
    use crate::*;
    use crate::point::Axis;
    use crate::point::CoordinateError;
    #[test]
    fn test_accessing_value() {
        let t = Table::new();
        let val = t.value_in_point(&Point{ x: 1, y: 6}).unwrap();
        
        assert_eq!(val, 0u8)
    }

    #[test]
    fn test_x_axis_of_range_access() {
        let t = Table::new();

        let x_error = t.value_in_point(&Point{ x: 9, y: 5 }).unwrap_err();
        assert_eq!(x_error, CoordinateError::OutOfRange(Axis::X));
    }

    #[test]
    fn test_y_axis_of_range_access() {
        let t = Table::new();

        let y_error = t.value_in_point(&Point{ x: 1, y: 9 }).unwrap_err();    
        assert_eq!(y_error, CoordinateError::OutOfRange(Axis::Y));
    }

    #[test]
    fn test_set_value_in_point() {
        let t = Table::new();
        let p = Point{ x: 0, y: 3 };
        let v = 5u8;

        let new_table = t.set_in_point(&p, v).unwrap();
        assert_eq!(new_table.as_ref().value_in_point(&p).unwrap(), v);

        let p2 = Point{x: 8, y: 3}; 
        let v2 = 7u8;
        let new_table_2 = new_table.set_in_point(&p2, v2).unwrap();
        assert_eq!(new_table_2.as_ref().value_in_point(&p2).unwrap(), v2);
        assert_eq!(new_table_2.as_ref().value_in_point(&p).unwrap(), v);
        assert_eq!(t.value_in_point(&p).unwrap(), 0);
        assert_eq!(t.value_in_point(&p2).unwrap(), 0);
    }

    #[test]
    fn test_debug_print() {
        let t = Table::new();
        println!("{:?}", t);
    }

    #[test]
    fn test_points_in_zone() {
        let table = Table::new();
        let points_in_zone = table.points_in_zone(&Point{x: 2, y: 1}).unwrap();
        assert_eq!(points_in_zone.len(), 9);
        assert_eq!(points_in_zone[0], Point{x: 0, y: 0});
        assert_eq!(points_in_zone[1], Point{x: 0, y: 1});
        assert_eq!(points_in_zone[2], Point{x: 0, y: 2});
        assert_eq!(points_in_zone[3], Point{x: 1, y: 0});
        assert_eq!(points_in_zone[4], Point{x: 1, y: 1});
        assert_eq!(points_in_zone[5], Point{x: 1, y: 2});
        assert_eq!(points_in_zone[6], Point{x: 2, y: 0});
        assert_eq!(points_in_zone[7], Point{x: 2, y: 1});
        assert_eq!(points_in_zone[8], Point{x: 2, y: 2});

        let table = Table::new();
        let points_in_zone = table.points_in_zone(&Point{x: 7, y: 3}).unwrap();
        assert_eq!(points_in_zone.len(), 9);
        assert_eq!(points_in_zone[0], Point{x: 6, y: 3});
        assert_eq!(points_in_zone[1], Point{x: 6, y: 4});
        assert_eq!(points_in_zone[2], Point{x: 6, y: 5});
        assert_eq!(points_in_zone[3], Point{x: 7, y: 3});
        assert_eq!(points_in_zone[4], Point{x: 7, y: 4});
        assert_eq!(points_in_zone[5], Point{x: 7, y: 5});
        assert_eq!(points_in_zone[6], Point{x: 8, y: 3});
        assert_eq!(points_in_zone[7], Point{x: 8, y: 4});
        assert_eq!(points_in_zone[8], Point{x: 8, y: 5});
    }
}