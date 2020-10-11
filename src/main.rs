pub mod table;
pub mod point;
pub mod validators;
pub mod point_selection;
use point_selection::empty_point_search::*;
use point_selection::line_point_selection::*;
use validators::sequence_validator::*;
use validators::TableValidator;
use table::Table;
use point::Point;
use point::CoordinateError;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};

static NTHREADS: i32 = 8;


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
    let (tx, rx): (Sender<Table>, Receiver<Table>) = mpsc::channel();
    let (txDone, rxDone): (Sender<Table>, Receiver<Table>) = mpsc::channel();
    let mut handles = vec![];
    let rxMutex = Arc::new(Mutex::new(rx));

    for id in 0..8 {
        let receiver = Arc::clone(&rxMutex);
        let sender = tx.clone();
        let doneSender = txDone.clone();
        let point_search = EmptyPointSearch{};
        
        let handle = thread::spawn(move || {
            loop {
                let table = receiver.lock().unwrap().recv().unwrap();
                let next_empty_point = point_search.next_empty(&table, SearchDirection::TopLeftBottom);
                match next_empty_point {
                    Option::None => {
                        doneSender.send(table);
                        return;
                    }
                    _ => (),
                }
                let next_empty_point = next_empty_point.unwrap();
                let sequence_validator = SequenceValidator{};
                let line_point_selection = LinePointSelection{};
                let zone_points = table.points_in_zone(&next_empty_point).unwrap();
                let zone_possibilites = sequence_validator.get_possibilites(&table, zone_points.iter().map(|f| f).collect());
                let vertical_points = line_point_selection.get_points(&table, &next_empty_point, SelectionType::Vertical).unwrap();
                let horizontal_points = line_point_selection.get_points(&table, &next_empty_point, SelectionType::Horizontal).unwrap();
                let vertical_possibilites = sequence_validator.get_possibilites(&table, vertical_points.iter().map(|f| f).collect());
                let horizontal_possibilites = sequence_validator.get_possibilites(&table, horizontal_points.iter().map(|f| f).collect());
                let common_possibilites = zone_possibilites.iter()
                    .filter(|v| vertical_possibilites.iter().any(|c| c == *v))
                    .filter(|v| horizontal_possibilites.iter().any(|c| c == *v))
                    .collect::<Vec<&u8>>();

                if common_possibilites.len() == 0 {
                } else {
                    for p in common_possibilites {
                    
                        let next_table = table.set_in_point(&next_empty_point, *p).unwrap();
                        sender.send(*next_table).unwrap();
                    }
                }
            } 
        });
        handles.push(handle);
    }
    let mut t = Table::new_from(
       [
            [0,0,0,0,0,0,0,0,0],
            [0,0,0,0,0,3,0,8,5],
            [0,0,1,0,2,0,0,0,0],
            [0,0,0,5,0,7,0,0,0],
            [0,0,4,0,0,0,1,0,0],
            [0,9,0,0,0,0,0,0,0],
            [5,0,0,0,0,0,0,7,3],
            [0,0,2,0,1,0,0,0,0],
            [0,0,0,0,4,0,0,0,9],
        ]);
    
    println!("{:?}", t);
    tx.send(t);


    for handle in handles {
        println!("{:?}", rxDone.recv().unwrap());
    }
}




