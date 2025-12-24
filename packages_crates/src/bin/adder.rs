use packages_crates::lib::add;

fn main() {
    let val1: i32 = 34;
    let val2: i32 = 74;
    println!(
        "adding from adder {} and {} = {}",
        val1,
        val2,
        add(val1, val2)
    )
}
