mod input;
mod emoji;
mod ui;
use std::io::Error;
use crate::input::Arrow;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crate::{input::Input, ui::start_ui};

const PATH_EMOJI: &str = include_str!("../asset/data-by-emoji-category.json");

#[tokio::main]
async fn main() -> Result<(), Error>{
    let emojis_hash = emoji::extract_emoji(PATH_EMOJI);
    let all_categories = ["All", "Smileys & Emotion", "People & Body", "Animals & Nature", "Food & Drink", "Travel & Places",
    "Activities", "Objects", "Symbols", "Flags"];


    let input = Input::new();
    let arrow = Arrow::new();

    enable_raw_mode()?;

    start_ui(emojis_hash, all_categories, input, arrow).await?;

    disable_raw_mode()
}
