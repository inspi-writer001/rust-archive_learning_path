#[derive(Clone, Debug)]
pub struct ICarol {
    index: usize,
    lyrics: Vec<String>,
}

const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

fn main() {
    let gifts: [String; 12] = [
        "and A partridge in a pear tree".to_string(),
        "Two turtle doves".to_string(),
        "Three French hens".to_string(),
        "Four calling birds".to_string(),
        "Five gold rings".to_string(),
        "Six geese a-laying".to_string(),
        "Seven swans a-swimming".to_string(),
        "Eight maids a-milking".to_string(),
        "Nine ladies dancing".to_string(),
        "Ten lords a-leaping".to_string(),
        "Eleven pipers piping".to_string(),
        "Twelve drummers drumming".to_string(),
    ];

    let christmas_days: usize = 10;
    let mut array_of_carols: Vec<ICarol> = Default::default();

    for iter in 0..=christmas_days {
        let lyric = format!(
            "on the {} day of Christmas\nMy true love sent to me",
            DAYS[iter]
        );
        let mut lyric_array = vec![lyric.to_string()];
        let mut iter_item = ICarol {
            index: iter,
            lyrics: lyric_array.clone(),
        };
        if DAYS[iter] == "first" {
            lyric_array.push("A partridge in a pear tree".to_string());
            iter_item = ICarol {
                index: iter_item.index,
                lyrics: lyric_array,
            };
        } else {
            // lyric_array.push(gifts[iter].clone().into());
            let mut backwards_iterator: usize = iter.clone();

            loop {
                lyric_array.push(gifts[backwards_iterator].clone());
                if backwards_iterator > 0 {
                    backwards_iterator = backwards_iterator - 1;
                } else {
                    break;
                }
            }
            iter_item = ICarol {
                index: iter_item.index,
                lyrics: lyric_array,
            };
        }
        array_of_carols.push(iter_item);
    }

    // println!("{:#?}", array_of_carols);

    for carol in &array_of_carols {
        println!("Day {}:", carol.index + 1);
        for line in &carol.lyrics {
            println!("{}", line);
        }
        println!("");
    }
}
