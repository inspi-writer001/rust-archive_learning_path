pub fn join() {
    #[derive(Debug)]
    struct IGameCharacter {
        uid: u32,
        is_strong: bool,
        demeanor: u32,
        health: u8,
    }

    let playable_character: IGameCharacter = IGameCharacter {
        uid: 100245,
        is_strong: false,
        demeanor: 8,
        health: 200,
    };

    let new_user = IGameCharacter {
        uid: 23874,
        ..playable_character
    };

    println!("{:#?}", playable_character);
    println!("{:#?}", new_user);

    let name: &str = "Blinky Gala";

    let few_characters: String = String::from(name);
    let same_characters: String = name.to_string();
    println!("{:?}", few_characters);
    println!("{:?}", same_characters);
    println!("{:?}", name);

    let mut index_to_query: usize = name.len() - 1;

    let some_first_three = &name[0..3];

    println!("some_first_three {:?}", some_first_three);

    match name.chars().nth(index_to_query) {
        Some(character_at_index) => {
            println!("{:?}", character_at_index)
        }
        None => {
            println!("No Character at index {}", index_to_query)
        }
    }

    index_to_query += 1;

    if let Some(character_at_index) = name.chars().nth(index_to_query) {
        print!("{:?}", character_at_index);
    } else {
        println!("No Character at index {}", index_to_query)
    }

    match name.chars().nth(index_to_query) {
        Some(some_value) => {
            println!("{}", some_value);
        }
        None => {
            println!("No value at index {}", index_to_query)
        }
    }



}
