pub fn loop_examples() {
    // print first 10 positive integers

    let mut i = 0;
    loop {
        if i < 10 {
            i += 1;
            println!("{i}");
        } else {
            break;
        }
    }
}
