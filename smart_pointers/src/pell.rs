use std::cell::UnsafeCell;

#[derive(Debug)]
pub struct Pell<T> {
    value: UnsafeCell<T>,
}

pub trait IPell<T> {
    fn new(value: T) -> Self;
    fn get(&self) -> T
    where
        T: Copy;
    fn set(&self, new_value: T);
}

// unsafe impl<T> Sync for Pell<T> {}

impl<T> IPell<T> for Pell<T> {
    fn new(value: T) -> Self {
        Pell {
            value: UnsafeCell::new(value),
        }
    }

    fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { *self.value.get() }
    }

    fn set(&self, new_value: T) {
        unsafe {
            *self.value.get() = new_value;
        }
    }
}

#[cfg(test)]
mod test_pell {

    use std::sync::Arc;
    use std::thread;

    use super::IPell;

    use super::Pell;

    #[test]
    fn test_pell() {
        let new_value = Pell::new(79);
        assert_eq!(new_value.get(), 79, "Cell wasn't created properly");

        new_value.set(94);
        assert_eq!(new_value.get(), 94, "Cell wasn't created properly");
    }

    // #[test]
    // fn test_pell_in_thread() {
    //     let first_attempt = Arc::new(Pell::new(vec![23, 98, 367]));

    //     let cl1 = Arc::clone(&first_attempt);
    //     let cl2 = Arc::clone(&first_attempt);

    //     let thread_1 = thread::spawn(move || {
    //         cl1.set(vec![34; 100]);
    //     });
    //     let thread_2 = thread::spawn(move || {
    //         cl2.set({
    //             unsafe {
    //                 let value = &mut *cl2.value.get();
    //                 let val_len = &value.len();

    //                 for i in 0..*val_len {
    //                     value[i] = value[i] + 87;
    //                 }

    //                 value.clone()
    //             }
    //         });
    //     });

    //     thread_1.join().unwrap();
    //     thread_2.join().unwrap();

    //     unsafe {
    //         println!(
    //             "current value for Pell data in array {:?}",
    //             *first_attempt.value.get()
    //         );
    //     }
    // }
}
