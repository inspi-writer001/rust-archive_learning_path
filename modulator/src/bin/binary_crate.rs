use modulator::dependency::mod_one::one;
use modulator::secret;
use std::cmp;

fn main() {
    println!("Binary crate: {}", modulator::secret::f1());
    println!("Dependency from Binary crate: {}", secret::f1());

    println!("one in bin from dependency: {}", one());

    println!(
        "{:?} {:?} {:?}",
        cmp::Ordering::Less,
        cmp::Ordering::Greater,
        cmp::Ordering::Equal
    );
}
