mod input;
mod emoji;
mod ui;
use std::fs::File;
use simple_home_dir::*;
use std::io::ErrorKind;

fn main() {
    let home_dir = home_dir().unwrap();
    let home_dir = home_dir.to_str();

    //reading in the emojis
    let path_emoji = format!(
        "{}/.cache/termiji/data-by-emoji-category.json",
        home_dir.unwrap()
    );

    let path = match File::open(&path_emoji) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => panic!("Please, make sure the emoji json file exists at {}.", path_emoji),
            _ => panic!("Error, trying to access emoji json file.")
        }
    };
    
    let emojis_hash = emoji::extract_emoji(path);
    let mut all_categories: Vec<String> = Vec::new();

    for key in emojis_hash.keys() {
        if !all_categories.contains(key) {
            all_categories.push(key.clone());
        }
    }

    all_categories.sort();
    for category in all_categories {
        println!("{}", category);
    }

    // for (category, emojis) in emojis_hash {
    //     println!("Category: {}", category);
    //
    //     for emoji in emojis {
    //         println!("Emoji: {}, {}, {}", emoji.get_emoji(), emoji.get_slug(), emoji.get_group());
    //     }
    // }
}
