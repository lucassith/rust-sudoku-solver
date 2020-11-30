pub mod table;
pub mod point;
pub mod validators;
pub mod fillers;
pub mod point_selection;
use fillers::simple_filler::SimpleFiller;
use fillers::Filler;
use table::Table;
use point_selection::empty_point_search::{EmptyPointSearch, SearchDirection};
use point::Point;
use point::CoordinateError;
use std::thread;
use std::time::{Instant};
use crossbeam_channel::{bounded, unbounded, select};

static NTHREADS: i32 = 32;


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

fn main() {
    let (tx, rx) = unbounded::<Table>();
    let (tx_done, rx_done) = unbounded::<Table>();
    let (tx_stop, rx_stop) = bounded::<bool>(1);
    let mut handles = vec![];

    for _ in 0..NTHREADS {
        let point_search = EmptyPointSearch{};
        let rx_stop_clone = rx_stop.clone();
        let rx_clone = rx.clone();
        let tx_clone = tx.clone();
        let tx_done_clone = tx_done.clone();
        
        
        let handle = thread::spawn(move || {
            loop {
                let table: Table;
                match select!(
                    recv(rx_stop_clone) -> _ => Option::None,
                    recv(rx_clone) -> table => Option::Some(table),
                ) {
                    Option::None => {
                        return;
                    }
                    Option::Some(x) => {
                        table = x.unwrap();
                    }
                }
                
                let next_empty_point = point_search.next_empty(&table, SearchDirection::TopLeftRight);
                match next_empty_point {
                    Option::None => {
                        tx_done_clone.send(table).unwrap();
                        return;
                    }
                    _ => (),
                }
                let next_empty_point = next_empty_point.unwrap();
                let filler = SimpleFiller{};
                let tables = filler.fill::<Table>(&table, &next_empty_point);
                match tables {
                    Option::Some(tableVec) => for t in tableVec {
                        tx_clone.send(t).unwrap();
                    }
                    Option::None => (),
                }
                
            } 
        });
        handles.push(handle);
    }
    let t = Table::new_from(
       [
            [8,6,0,0,2,0,0,0,0],
            [2,0,0,7,0,0,0,5,9],
            [5,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,0,8,0,0],
            [0,4,0,0,9,0,0,0,0],
            [0,0,5,3,0,0,0,0,7],
            [0,0,0,0,0,0,0,0,0],
            [0,2,0,0,0,0,6,0,0],
            [0,0,7,0,0,9,0,0,0]
        ]);
    println!("{:?}", t);
    let start = Instant::now();
    tx.send(t).unwrap();
    println!("{:?}", rx_done.recv().unwrap());
    println!("Sudoku solved in: {:?}", start.elapsed());
    for _ in handles {
        tx_stop.send(true).unwrap();
    }
}




