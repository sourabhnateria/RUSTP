use rand::prelude::*;
use std::io;

fn main() {
    let guess_list = [
        "violet", "indigo", "blue", "green", "yellow", "orange", "red",
    ];
    let mut rng = thread_rng();

    let index = rng.gen_range(0..guess_list.len());
    let random_color = guess_list[index];
    println!("random color : {} ", random_color);
    let mut input = String::new();

    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let _color_selected = input.trim().to_lowercase();
                println!("color selected:{}", _color_selected);

                if !guess_list.contains(&_color_selected.as_str()) {
                    println!("color entered does not found");
                    continue;
                }
                if guess_checker(&_color_selected, random_color) {
                    println!("you are winner");
                    break;
                } else {
                    println!("retry");
                }
            }
            Err(error) => {
                println!("error : {}", error)
            }
        }
    }
}

fn guess_checker(guessed_color: &str, random_selected: &str) -> bool {
    return guessed_color == random_selected;
}
