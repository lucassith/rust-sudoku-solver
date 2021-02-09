use std::future::Future;
use crate::table::Table;
use std::task::Context;
use std::pin::Pin;
use std::task::Poll;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use std::thread::sleep;
use tokio::time::Duration;

pub struct TableBus {
    tables: Arc<Mutex<VecDeque<Table>>>,
}

impl TableBus {
    pub fn new() -> TableBus {
        TableBus { tables: Arc::new( Mutex::new( VecDeque::new() )) }
    }

    pub fn count(&self) -> usize {
        self.tables.lock().unwrap().len()
    }

    pub fn insert(&self, table: Table) {
        self.tables.lock().unwrap().push_front(table)
    }

    pub async fn receive(&self) -> Option<Table> {
        for _ in 0..10 {
            let t = self.tables.lock().unwrap().pop_front();
            if t.is_some() {
                return t
            }
            sleep(Duration::from_millis(50))
        }
        return  Option::None
    }
}

impl Clone for TableBus {
    fn clone(&self) -> Self {
        TableBus { tables: self.tables.clone() }
    }

    fn clone_from(&mut self, source: &Self) {
        TableBus { tables: source.tables.clone() };
    }
}

impl Future for Table {
    type Output = Table;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(self.clone())
    }
}


#[cfg(test)]
mod test {
    use crate::bus::table_bus::TableBus;
    use crate::table::Table;
    use crate::Settable;
    use crate::point::Point;

    #[tokio::test]
    async fn test_polling() {
        let tb = TableBus::new();
        let mut t1 = Table::new();
        t1 = *t1.set_in_point(&Point{ x: 1, y: 2 }, 1).unwrap();
        t1 = *t1.set_in_point(&Point{ x: 3, y: 2 }, 1).unwrap();
        let t2 = *t1.set_in_point(&Point{ x: 3, y: 6 }, 1).unwrap();
        tb.insert(t1);
        tb.insert(t2);
        println!("{:?}", tb.receive().await.unwrap().await);
        println!("{:?}", tb.receive().await.unwrap().await);
    }
}
