use fillers::Filler;
use fillers::simple_filler::SimpleFiller;
use point::CoordinateError;
use point::Point;
use point_selection::empty_point_search::{EmptyPointSearch, SearchDirection};
use table::Table;

use crate::bus::table_bus::TableBus;
use signal_hook::iterator::Signals;
use signal_hook::consts::{SIGINT};
use std::process;

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
            [1,0,0,0,0,7,0,9,0],
            [0,3,0,0,2,0,0,0,8],
            [0,0,9,6,0,0,5,0,0],
            [0,0,5,3,0,0,9,0,0],
            [0,1,0,0,8,0,0,0,2],
            [6,0,0,0,0,4,0,0,0],
            [3,0,0,0,0,0,0,1,0],
            [0,4,0,0,0,0,0,0,7],
            [0,0,7,0,0,0,3,0,0]
        ]);
    println!("{:?}", t);
    table_bus.insert(t);

    let mut signals = Signals::new(&[SIGINT]).unwrap();
    let signal_solution_bus = solution_bus.clone();
    let ctrlchandle = tokio::spawn(async move {
        for _ in signals.forever() {
            let count = signal_solution_bus.count();
            for _ in 0..count {
                println!("{:?}", signal_solution_bus.receive().await.unwrap().await);
            }
            println!("There are {} solutions...", count);
            process::exit(0x1)
        }
    });

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
                if next_empty_point == None {
                    continue;
                }
                let next_empty_point = next_empty_point.unwrap();
                let filler = SimpleFiller{};
                let tables = filler.fill::<Table>(&table, &next_empty_point);
                match tables {
                    Some(table_vec) => for t in table_vec {
                        tb.insert(t)
                    }
                    None => continue,
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
    ctrlchandle.abort();
    process::exit(0)
}




