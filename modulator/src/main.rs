use modulator::dependency;
use modulator::f3;
use modulator::secret;

fn main() {
    let secret1 = secret::f1();
    // secret::f2(); --> privae fn

    let secret2 = f3();

    println!("data: {} {}", secret1, secret2);
    println!("one from dependency: {}", dependency::mod_one::one());
}
