use std::io;

pub fn match_fn() {
    let lucky_number: u32 = 7;
    let max_tries: u32 = 3;
    let mut user_tries: u32 = 0;

    loop {
        let mut user_response = String::new();
        if user_tries >= max_tries {
            println!("EXCEEDED TRIALS. Game Over!");
            break;
        }
        println!("Enter a lucky number: ");
        io::stdin()
            .read_line(&mut user_response)
            .expect("Failed to read line");

        match user_response.trim().parse::<u32>() {
            Ok(guess) => {
                if guess != lucky_number {
                    println!("Wrong guess... Enter a lucky number: ");
                } else {
                    println!("Yay,,, you won! {} was correct", user_response.trim());
                    break;
                }
            }
            Err(_) => {
                println!("Invalid Input");
            }
        }

        user_tries += 1;
    }
}
