use std::{fs::File, io::Read};
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct Emoji {
    emoji: String,
    slug: String,
    group: String,
}

impl Emoji {
    pub fn get_emoji(&self) -> &str {
        &self.emoji
    }

    pub fn get_slug(&self) -> &str {
        &self.slug
    }

    pub fn get_group(&self) -> &str {
        &self.group
    }
}

pub fn extract_emoji(mut path: File) -> HashMap<String, Vec<Emoji>> {
    let mut data = String::new();
    path.read_to_string(&mut data).unwrap();
    let mut emojis: Vec<Emoji> = serde_json::from_str(&data).unwrap();

    let mut emoji_categories = HashMap::new();


    for emoji in &emojis {
        emoji_categories.entry(emoji.group.clone())
            .or_insert_with(Vec::new)
            .push(emoji.clone());
    }

    emojis.sort();
    emoji_categories.insert("All".to_string(), emojis.clone());

    emoji_categories
}
