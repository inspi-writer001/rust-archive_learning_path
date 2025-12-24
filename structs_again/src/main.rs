#[derive(Clone, Debug)]
struct Boy {
    age: i32,
    name: String,
    friends: Option<Vec<String>>,
}

trait BoyTrait {
    fn say_name(&self) -> String;
    fn add_to_friends(&mut self, new_friend: String);
    fn list_friends(&self) -> Vec<String>;
}

impl BoyTrait for Boy {
    fn say_name(&self) -> String {
        format!("Hello I am {}", self.name)
    }

    fn add_to_friends(&mut self, new_friend: String) {
        match &mut self.friends {
            Some(friends) => friends.push(new_friend),
            None => self.friends = Some(vec![new_friend]),
        };
    }

    fn list_friends(&self) -> Vec<String> {
        match &self.friends {
            Some(friends) => friends.to_vec(),
            None => Vec::new(),
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut tunde: Boy = Boy {
        age: 24,
        name: "Tunde".to_string(),
        friends: None,
    };

    println!("{}", tunde.say_name());
    tunde.add_to_friends("Bello".to_string());
    tunde.add_to_friends("Pius".to_string());
    tunde.add_to_friends("Paul".to_string());
    println!("{:?}", tunde.list_friends());

    if let Some(ref friends) = tunde.friends {
        for friend in friends {
            println!("Friend: {:?}", friend);
        }
    }

    let remove_index: usize = 4;

    match tunde.friends.as_mut() {
        Some(friends) => match friends.get(remove_index) {
            Some(friend) => {
                println!("Removing: {:?}", friend);
                friends.remove(remove_index);
            }
            None => {
                println!("index at {} doesn't exist", remove_index);
            }
        },
        None => {
            println!("No friends to remove from!");
        }
    }

    match tunde.friends.as_mut() {
        Some(friends_vec) => {
            for a_friend in friends_vec {
                println!("{:?}", a_friend);
            }
        }
        None => {
            println!("No friends to iterate");
        }
    }
}
