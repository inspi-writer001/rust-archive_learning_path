use core::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

use crate::cell::Cell;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PefState {
    Unshared,
    Shared(u8),
    Exclusive,
}

pub struct PefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<PefState>,
}

impl<T> PefCell<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(PefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            PefState::Shared(n) => {
                self.state.set(PefState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            PefState::Unshared => {
                self.state.set(PefState::Shared(1));
                Some(Ref { refcell: self })
            }
            PefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let PefState::Unshared = self.state.get() {
            self.state.set(PefState::Exclusive);
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

pub struct Ref<'info, T> {
    refcell: &'info PefCell<T>,
}

pub struct RefMut<'info, T> {
    refcell: &'info PefCell<T>,
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            PefState::Unshared | PefState::Exclusive => {
                println!("did it get here");
                unreachable!()
            }
            PefState::Shared(1) => {
                println!("dropping state from 1");
                self.refcell.state.set(PefState::Unshared);
            }
            PefState::Shared(n) => {
                println!("dropping state from {:?}", n);
                self.refcell.state.set(PefState::Shared(n - 1))
            }
        }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            PefState::Exclusive => {
                println!("dropping mutable state");
                self.refcell.state.set(PefState::Unshared);
            }
            PefState::Shared(_) | PefState::Unshared => {
                unreachable!()
            }
        }
    }
}

#[cfg(test)]
mod test_pef {
    use core::borrow;

    use crate::pefcell::{PefCell, PefState};

    #[derive(Copy, Clone, Debug)]
    struct Person<'a> {
        pub first_name: &'a str,
        pub age: u8,
    }

    #[test]
    fn init_pef() {
        const SOME_AMAZING_VALUE: PefCell<Person<'_>> = PefCell::new(Person {
            first_name: "Paul",
            age: 12,
        });

        let new_borrow = SOME_AMAZING_VALUE;
        let first_borrowing = new_borrow.borrow();
        let second_borrowing = new_borrow.borrow();
        let third_borrowing = new_borrow.borrow();

        if let Some(value) = SOME_AMAZING_VALUE.borrow() {
            assert_eq!(
                new_borrow.state.get(),
                PefState::Shared(3),
                "borrows failed"
            );
            drop(first_borrowing); // dropping state from 3
            drop(second_borrowing); // dropping state from 2
            drop(third_borrowing); // dropping state from 1
            let new_guy: Option<crate::pefcell::RefMut<'_, Person<'_>>> =
                value.refcell.borrow_mut();
            if let Some(mut mut_value) = new_guy {
                mut_value.first_name = "Stephen";
            }
            println!("person name is {:?}", value.first_name);
        }

        assert_eq!(
            SOME_AMAZING_VALUE.state.get(),
            PefState::Unshared,
            "Dropping failed"
        );
        println!("person state is {:?}", SOME_AMAZING_VALUE.state.get());
    }
}
