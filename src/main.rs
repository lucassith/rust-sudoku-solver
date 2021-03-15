use fillers::Filler;
use fillers::simple_filler::SimpleFiller;
use point::CoordinateError;
use point::Point;
use point_selection::empty_point_search::{EmptyPointSearch, SearchDirection};
use table::Table;

use crate::bus::table_bus::TableBus;
use std::sync::{Arc, Mutex};
use std::ops::Add;

pub mod table;
pub mod point;
pub mod validators;
pub mod fillers;
pub mod point_selection;
pub mod bus;

static NTHREADS: i32 = 16;

pub trait Selectable {
    fn value_in_point(&self, point: &Point) -> Result<u8, CoordinateError>;
}

pub trait Settable {
    fn set_in_point(&self, point: &Point, value: u8) -> Result<Box<Self>, CoordinateError>;
}

pub trait SudokuTable {
    fn points_in_zone(&self, point: &Point) -> Result<Vec<Point>, CoordinateError>;
    fn possible_values(&self) -> Vec<u8>;
}

pub trait SquareTable {
    fn dimensions(&self) -> usize;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut handles = vec![];
    let table_bus = TableBus::new();
    let solution_bus = TableBus::new();
    let t = Table::new_from(
        [
            [0,0,9,2,1,8,0,0,0],
            [1,7,0,0,9,6,8,0,0],
            [0,4,0,0,5,0,0,0,6],
            [4,5,1,0,6,0,3,7,0],
            [0,0,0,0,0,5,0,0,9],
            [9,0,2,3,7,0,5,0,0],
            [6,0,0,5,0,1,0,0,0],
            [0,0,0,0,4,9,2,5,7],
            [0,9,4,8,0,0,0,1,3]
        ]);
    println!("{:?}", t);
    table_bus.insert(t);
    for _ in 0..NTHREADS {
        let point_search = EmptyPointSearch{};
        let tb = table_bus.clone();
        let sb = solution_bus.clone();
        let handle = tokio::spawn(async move {
            loop {
                let table = match tb.receive().await {
                    Some(future) => future.await,
                    None => {
                        return;
                    }
                };
                if table.is_filled() {
                    sb.insert(table);
                    continue;
                }
                let next_empty_point = point_search.next_empty(&table, SearchDirection::BottomRightTop);
                match next_empty_point {
                    Option::None => {
                        continue;
                    }
                    _ => (),
                }
                let next_empty_point = next_empty_point.unwrap();
                let filler = SimpleFiller{};
                let tables = filler.fill::<Table>(&table, &next_empty_point);
                match tables {
                    Option::Some(table_vec) => for t in table_vec {
                        tb.insert(t)
                    }
                    Option::None => continue,
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await?;
    }
    let count = solution_bus.count();
    for _ in 0..solution_bus.count() {
        println!("{:?}", solution_bus.receive().await.unwrap().await);
    }
    println!("There are {} solutions...", count);
    return Result::Ok(())
}




