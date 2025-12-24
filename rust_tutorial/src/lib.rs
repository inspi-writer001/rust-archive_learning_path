pub mod basic {
    pub mod maino;
}

pub mod strings {
    pub mod strings;
}

pub mod loops;
pub mod match_fn;

pub fn main() {
    basic::maino::maino();
    strings::strings::join();
    match_fn::match_fn();
    loops::loop_examples(); 
}
