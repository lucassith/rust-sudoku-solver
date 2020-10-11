use crate::point::Point;
use crate::point::CoordinateError;
use crate::point::Axis;
use crate::SquareTable;

pub struct LinePointSelection {
    
}

pub enum SelectionType {
    Vertical,
    Horizontal
}

impl LinePointSelection {
    pub fn get_points(&self, table: &impl SquareTable, point: &Point, selection_type: SelectionType) -> Result<Vec<Point>, CoordinateError> {
        if point.x > table.dimensions() {
            return Result::Err(CoordinateError::OutOfRange(Axis::X))
        }
        if point.y > table.dimensions() {
            return Result::Err(CoordinateError::OutOfRange(Axis::Y))
        }
        let mut points = Vec::new();
        match selection_type {
            SelectionType::Vertical => {
                for i in 0..table.dimensions() {
                    points.push(Point{x: point.x, y: i})
                }
            }
            SelectionType::Horizontal => {
                for i in 0..table.dimensions() {
                    points.push(Point{x: i, y: point.y})
                }
            }
        }
        return Result::Ok(points);
    }
}

#[cfg(test)]
mod test {
    use crate::point::Point;
    use super::*;
    use crate::SquareTable;
    use crate::point::CoordinateError;
    use crate::point::Axis;

    struct MockTable {

    }

    impl SquareTable for MockTable {
        fn dimensions(&self) -> usize {
            return 4usize;
        }
    }
    #[test]
    fn test_return_horizontal_points() {
        
        let t = MockTable{};
        let s = LinePointSelection{};

        let y = s.get_points(&t, &Point{ x: 1, y: 3 }, SelectionType::Horizontal).unwrap();
        assert_eq!(y.len(), 4);
        assert_eq!(y[0], Point{x: 0, y: 3});
        assert_eq!(y[1], Point{x: 1, y: 3});
        assert_eq!(y[2], Point{x: 2, y: 3});
        assert_eq!(y[3], Point{x: 3, y: 3});
    }

    #[test]
    fn test_return_vertical_points() {
        
        let t = MockTable{};
        let s = LinePointSelection{};

        let y = s.get_points(&t, &Point{ x: 1, y: 3 }, SelectionType::Vertical).unwrap();
        assert_eq!(y.len(), 4);
        assert_eq!(y[0], Point{x: 1, y: 0});
        assert_eq!(y[1], Point{x: 1, y: 1});
        assert_eq!(y[2], Point{x: 1, y: 2});
        assert_eq!(y[3], Point{x: 1, y: 3});
    }

    #[test]
    fn test_return_error_if_out_of_boundaries() {
        
        let t = MockTable{};
        let s = LinePointSelection{};

        let y = s.get_points(&t, &Point{ x: 1, y: 5 }, SelectionType::Vertical).unwrap_err();
        assert_eq!(y, CoordinateError::OutOfRange(Axis::Y));

        let x = s.get_points(&t, &Point{ x: 5, y: 2 }, SelectionType::Vertical).unwrap_err();
        assert_eq!(x, CoordinateError::OutOfRange(Axis::X));
    }
}

