mod input;
mod emoji;
mod ui;
use std::{fs::File, io::Error};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use simple_home_dir::*;
use std::io::ErrorKind;

use crate::{input::Input, ui::start_ui};

#[tokio::main]
async fn main() -> Result<(), Error>{
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
    let all_categories = ["All", "Smileys & Emotion", "People & Body", "Animals & Nature", "Food & Drink", "Travel & Places",
    "Activities", "Objects", "Symbols", "Flags"];


    let input = Input::new();

    enable_raw_mode()?;

    start_ui(emojis_hash, all_categories, input).await?;

    disable_raw_mode()
}
