use crate::emoji::Emoji;
use crossterm::event::{Event, KeyCode};
use std::{collections::HashMap};
use terminal_size::{Width, Height, terminal_size};
use cli_clipboard::{ClipboardContext, ClipboardProvider};

pub struct Input {
    buffer: String,
    matched_emojis: Vec<Emoji>,
    exit: bool,
    copy: bool,
}

pub struct Arrow {
    cur_position: u32,
    start_point: u16,
    end_point: u16,
    cur_category: usize,
    width: u16,
}

impl Input {
    pub fn new() -> Input {
        Input {
            buffer: String::new(),
            matched_emojis: Vec::new(),
            exit: false,
            copy: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn get_exit_status(&self) -> bool {
        self.exit
    }

    pub fn get_buffer(&self) -> &String {
        &self.buffer
    }

    pub fn add_to_buffer(&mut self, ch: char) {
        self.buffer.push(ch);
    }

    pub fn pop_buffer(&mut self) {
        if !self.buffer.is_empty()  {
            self.buffer.remove(self.buffer.len() - 1);
        }
    }

    pub fn add_to_matched(&mut self, emoji: Emoji) {
        self.matched_emojis.push(emoji);
    }

    pub fn get_matched(&self) -> &Vec<Emoji> {
        &self.matched_emojis
    }

    pub fn clear_matched(&mut self) {
        self.matched_emojis.clear();
    }

    pub fn get_copied(&self) -> bool {
        self.copy
    }
}

impl Arrow {
    pub fn new() -> Arrow {
        let terminal_size = terminal_size();
        let height: u16;
        let width: u16;

        if let Some((Width(w), Height(h))) = terminal_size {
            height = h;
            width = w;
        } else {
            panic!("Something went wrong trying to get terminal size")
        }

        Arrow { 
            cur_position: 0,
            start_point: 0,
            end_point: height,
            cur_category: 0,
            width,
       }
    }

    pub fn increment_start_point(&mut self) {
        self.start_point = self.start_point.saturating_add(1);
    }

    pub fn decrement_start_point(&mut self) {
        self.start_point = self.start_point.saturating_sub(1); 
    }

    pub fn change_cur_category_left(&mut self) {
        if self.cur_category != 0 {
            self.cur_category = self.cur_category.saturating_sub(1);
        } else {
            self.cur_category = 9;
        }

        self.start_point = 0;
        self.cur_position = 0;
    }

    pub fn change_cur_category_right(&mut self) {
        if self.cur_category < 9 {
            self.cur_category = self.cur_category.saturating_add(1);
        } else {
            self.cur_category = 0;
        }

        self.start_point = 0;
        self.cur_position = 0;
    }

    pub fn get_start_point(&self) -> u16 {
        self.start_point
    }

    pub fn get_end_point(&self) -> u16 {
        self.end_point
    }

    pub fn get_width(&self) -> u16 {
        self.width
    }
    
    pub fn get_cur_category(&self) -> usize {
        self.cur_category
    }

    pub fn set_new_end_point(&mut self) {
        let size = terminal_size();
        if let Some((Width(w), Height(h))) = size {
            self.end_point = self.get_start_point() + h;
            self.width = w;
        }

        self.end_point = self.end_point.saturating_sub(5);
    }

    pub fn get_cur_pos(&self) -> u32 {
        self.cur_position
    }

    pub fn increment_pos(&mut self) {
        self.cur_position = self.cur_position.saturating_add(1);
    }

    pub fn decrement_pos(&mut self) {
        self.cur_position = self.cur_position.saturating_sub(1);
    }
}

pub fn start_input(event: Event, all_categories: [&str; 10], mut input: Input,
    emojis_hash: &HashMap<String, Vec<Emoji>>, mut arrow: Arrow) -> (Input, Arrow) {
    input.copy = false;

    if let Event::Key(key_event) = event {
        match key_event.code {
            KeyCode::Down => {
                if arrow.get_cur_pos() == (input.get_matched().len().saturating_sub(1)) as u32 && !input.get_matched().is_empty() {
                    arrow.cur_position = 0;
                    arrow.start_point = 0;
                } else if arrow.get_cur_pos() == arrow.get_end_point().saturating_sub(1) as u32 
                    && arrow.get_cur_pos() != (input.get_matched().len().saturating_sub(1)) as u32
                    && arrow.get_end_point() != (input.get_matched().len() as u16) {
                        arrow.increment_pos();
                        arrow.increment_start_point();
                } else {
                    arrow.increment_pos();
                }
            },
            KeyCode::Up => {
                if arrow.get_cur_pos() == arrow.get_start_point() as u32 && arrow.get_cur_pos() != 0 {
                    arrow.decrement_start_point();
                    arrow.decrement_pos();
                } else if arrow.get_cur_pos() == 0 && !input.get_matched().is_empty() {
                    arrow.cur_position = (input.get_matched().len().saturating_sub(1)) as u32;
                    arrow.start_point = (arrow.get_cur_pos() as u16).saturating_sub(arrow.get_end_point() - 1);
                } else if arrow.get_cur_pos() == 0 && input.get_matched().is_empty() {
                    arrow.cur_position = emojis_hash.get(all_categories[arrow.get_cur_category()]).unwrap().
                        iter().len().saturating_sub(1) as u32;
                    arrow.start_point = (arrow.get_cur_pos() as u16).saturating_sub(arrow.get_end_point() - 1);
                } else {
                    arrow.decrement_pos();
                }
            },
            KeyCode::Left => {
                arrow.change_cur_category_left();
            },
            KeyCode::Right => {
                arrow.change_cur_category_right();
            },
            KeyCode::Backspace => {
                input.pop_buffer();
                arrow.cur_position = 0;
                arrow.start_point = 0;
            },
            KeyCode::Enter => {
                let mut ctx = ClipboardContext::new().unwrap();
                let mut emoji_vec = input.get_matched();                

                if input.get_matched().is_empty() {
                    emoji_vec = emojis_hash.get(all_categories[arrow.get_cur_category()]).unwrap();
                } 

                let emoji = emoji_vec[arrow.get_cur_pos() as usize].get_emoji();
                ctx.set_contents(emoji.to_string()).unwrap();

                input.copy = true;
            },
            KeyCode::Esc => {
                input.exit();
            }
            KeyCode::Char(c) => {
                if c.is_whitespace() {
                    input.add_to_buffer(' ');
                    arrow.cur_position = 0;
                    arrow.start_point = 0;
                } else if c.is_ascii_graphic() {
                    input.add_to_buffer(c);
                    arrow.cur_position = 0;
                    arrow.start_point = 0;
                }
            },
            _ => {},

        }

        let selected_category = emojis_hash.get(all_categories[arrow.cur_category]).unwrap();

        input.clear_matched();

        if !input.get_buffer().is_empty() {
            for emoji in selected_category {
                if emoji.get_slug().contains(input.get_buffer()) {
                    input.add_to_matched(emoji.clone());
                }
            }
        } else {
            input.matched_emojis = emojis_hash.get(all_categories[arrow.get_cur_category()]).unwrap().to_vec();
        }
    }

    (input, arrow)
}
