use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        /// SAFETY: we know no-one else is concurrently mutating self.value (because !Sync)
        /// SAFETY: we know we're not overriding the values of the shared reference since we're not giving any out in get();
        unsafe {
            *self.value.get() = value
        };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }

    // pub fn get(&self) -> &T {
    //     unsafe { &*self.value.get() }
    // }
}

// TODO this should be removed for sanity
unsafe impl<T> Sync for Cell<T> {}

#[cfg(test)]
mod test {
    use std::{sync::Arc, thread};

    /// we're able to pass the Cell across threeads concurrently because we unsafely implemented Sync for Cell
    use super::Cell;
    #[test]
    fn bad() {
        let x = Arc::new(Cell::new(23));
        let x1 = Arc::clone(&x);
        let x2 = Arc::clone(&x);

        let y = Arc::new(Cell::new(0));
        let y1 = Arc::clone(&y);
        let y2 = Arc::clone(&y);

        let first_thread = thread::spawn(move || {
            x1.set(34);
        });

        let second_thread = thread::spawn(move || {
            x2.set(44);
        });

        first_thread.join().unwrap();
        second_thread.join().unwrap();
        println!("new x: {:?}", x.get());

        let ytd1 = thread::spawn(move || {
            for _ in 0..1_000_000 {
                let y = y1.get();
                y1.set(y + 1);
            }
        });

        let ytd2 = thread::spawn(move || {
            for _ in 0..1_000_000 {
                let y = y2.get();
                y2.set(y + 1);
            }
        });

        ytd1.join().unwrap();
        ytd2.join().unwrap();

        println!("total value should be 2mill 2_000_000: {:?}", y.get());
    }

    #[test]
    fn bad2() {
        let x = Cell::new(vec![43, 23]);

        let first = x.value.get();
        x.set(vec![]);

        unsafe { eprintln!("x: {:?}", *first) }
    }
}
