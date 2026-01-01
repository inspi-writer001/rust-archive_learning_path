pub mod cell;
pub mod pell;
pub mod refcell;

#[cfg(test)]
mod tests {
    use core::cell::Cell;
    use std::sync::Arc;

    /// Ignore this file, I originally should delete this test.. but yunno

    #[test]
    fn cell() {
        #[derive(Debug, Clone)]
        pub struct People {
            pub name: *const str,
            pub count: Arc<Cell<u64>>,
        }

        let collection_of_people = People {
            name: "Africans",
            count: Arc::new(Cell::new(23)),
        };

        collection_of_people.count.set(24);

        let first_getter = &collection_of_people.count;

        let x1 = Arc::clone(&first_getter);
        let x2 = Arc::clone(&first_getter);

        let people = unsafe { &*collection_of_people.name };

        // let f_thread = thread::spawn(||
        {
            x1.set(1000_000_000);
        }
        // );

        // let s_thread = thread::spawn(||
        {
            x2.set(2_000_000_000);
        }
        // );

        // f_thread.join().unwrap();
        // s_thread.join().unwrap();
        println!("{:?}", collection_of_people.count.get());
        println!("{:?}", collection_of_people);
        print!("{:?} ", people);
    }
}
