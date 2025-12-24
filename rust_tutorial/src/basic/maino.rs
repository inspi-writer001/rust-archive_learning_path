pub fn maino() {
    let num_1 = 0.2;
    let num_3 = 0.74;

    let max_u32 = std::u32::MAX;

    println!("{}", max_u32);

    println!("{}", num_1 + num_3);

    let ch1 = "X";
    let ch2 = "\u{2603}";

    println!("{}, {}", ch1, ch2);

    let first: f64 = 7.0;
    let second: f64 = 4.0;

    println!(
        "{} {} {} {} {}",
        first + second,
        first - second,
        first * second,
        first / second,
        first % second
    );

    let btc = "Bitcoin";
    let eth = "Ethereum";
    let sol = "Solana";

    println!("Normal order: {} {} {}", btc, eth, sol);
    println!("Flippening: {1} {0} {2} {2}", btc, eth, sol);
    println!("{btc} {eth} {sol}");
    println!(
        "{hey} {you} {rock}",
        hey = "Hey",
        you = "You",
        rock = "Rockkkk"
    );

    #[derive(Debug)]
    struct NewUser<'a> {
        name: &'a str,
        age: u8,
    }

    let new_user: NewUser = NewUser {
        name: "Tunde",
        age: 12,
    };

    println!("{:?}", new_user);
    println!("{:#?}", new_user); // pretty print

    let message = format!(
        "Kwargs: {solana} {ethereum} {bitcoin}",
        bitcoin = btc,
        ethereum = eth,
        solana = sol
    );
    println!("Look I've made this special {message}!", message = message);

    let cheat_code: [u32; 4] = [19, 65, 9, 17];
    let zeros = [0.0; 10];

    println!("Array: {:?}", cheat_code);
    println!("First element of the array: {}", cheat_code[0]);

    println!("Array(length: {}): {:?}", zeros.len(), zeros);

    let slice = &cheat_code[1..3];
    println!("Slice of cheat_code: {:?} {}", slice, slice.len());

    let fixed_array_of_zeros = [0; 12];
    println!(
        "Length of the Array is {} {:?}",
        fixed_array_of_zeros.len(),
        fixed_array_of_zeros
    );
}
