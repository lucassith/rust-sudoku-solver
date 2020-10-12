use crate::point::Point;
use crate::Selectable;
use crate::SquareTable;
pub enum SearchDirection {
    TopLeftBottom,
    TopLeftRight,
    BottomRightTop,
}

pub struct EmptyPointSearch {
    
}

impl EmptyPointSearch {
    pub fn next_empty(&self, table: &(impl Selectable + SquareTable), search_direction: SearchDirection) -> Option<Point> {
        match search_direction {
            SearchDirection::TopLeftBottom => {
                for x in 0..table.dimensions() {
                    for y in 0..table.dimensions() {
                        if table.value_in_point(&Point{x: x, y: y}).unwrap() == 0 {
                            return Option::Some(Point{x: x, y: y});
                        }
                    }
                }
            }
            SearchDirection::TopLeftRight => {
                for y in 0..table.dimensions() {
                    for x in 0..table.dimensions() {
                        if table.value_in_point(&Point{x: x, y: y}).unwrap() == 0 {
                            return Option::Some(Point{x: x, y: y});
                        }
                    }
                }
            }
            SearchDirection::BottomRightTop => {
                for y in (0..table.dimensions()).rev() {
                    for x in (0..table.dimensions()).rev() {
                        if table.value_in_point(&Point{x: x, y: y}).unwrap() == 0 {
                            return Option::Some(Point{x: x, y: y});
                        }
                    }
                }
            }
        }
        
        return Option::None;
    }
}


#[cfg(test)]
mod test {
    use crate::point::Point;
    use crate::point::CoordinateError;
    use super::*;
    use crate::Selectable;
    use crate::SquareTable;

    struct MockTable {

    }

    impl SquareTable for MockTable {
        fn dimensions(&self) -> usize {
            return 5usize;
        }
    }

    impl Selectable for MockTable {
        fn value_in_point(&self, point: &Point) -> Result<u8, CoordinateError> {
            if point.x == 0 && point.y == 1 {
                return Result::Ok(0);
            }
            if point.x == 1 && point.y == 0 {
                return Result::Ok(0);
            }
            if point.x == 4 && point.y == 3 {
                return Result::Ok(0);
            }
            return Result::Ok(1);
        }
    }

    #[test]
    fn test_return_search_top_left_bottom() {
        
        let t = MockTable{};
        let s = EmptyPointSearch{};

        let p = s.next_empty(&t, SearchDirection::TopLeftBottom).unwrap();
        assert_eq!(p, Point{x: 0, y:1});
    }

    #[test]
    fn test_return_search_top_left_right() {
        
        let t = MockTable{};
        let s = EmptyPointSearch{};

        let p = s.next_empty(&t, SearchDirection::TopLeftRight).unwrap();
        assert_eq!(p, Point{x: 1, y:0});
    }
}

