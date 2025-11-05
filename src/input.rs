use crate::emoji::Emoji;
use crossterm::event::{Event, KeyCode};
use std::{collections::HashMap};
use terminal_size::{Width, Height, terminal_size};

pub struct Input {
    buffer: String,
    cur_position: u32,
    start_point: u16,
    end_point: u16,
    cur_category: usize,
    exit: bool,
}

impl Input {
    pub fn new() -> Input {
        let size = terminal_size();
        let height: u16;
        if let Some((Width(_w), Height(h))) = size {
            height = h;
        } else {
            panic!("Something went wrong, trying to get the terminal size.");
        }

        Input {
            buffer: String::new(),
            cur_position: 0,
            start_point: 0,
            end_point: height,
            cur_category: 0,
            exit: false,
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn get_exit_status(&self) -> bool {
        self.exit
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
    }

    pub fn change_cur_category_right(&mut self) {
        if self.cur_category < 9 {
            self.cur_category = self.cur_category.saturating_add(1);
        } else {
            self.cur_category = 0;
        }

        self.start_point = 0;
    }

    pub fn get_start_point(&self) -> u16 {
        self.start_point
    }

    pub fn get_end_point(&self) -> u16 {
        self.end_point
    }
    
    pub fn get_cur_category(&self) -> usize {
        self.cur_category
    }

    pub fn get_buffer(&self) -> &String {
        &self.buffer
    }

    pub fn set_new_end_point(&mut self) {
        let size = terminal_size();
        if let Some((Width(_w), Height(h))) = size {
            self.end_point = self.get_start_point() + h;
        }
    }

    pub fn add_to_buffer(&mut self, ch: char) {
        self.buffer.push(ch);
    }

    pub fn pop_buffer(&mut self) {
        if !self.buffer.is_empty()  {
            self.buffer.remove(self.buffer.len() - 1);
        }
    }
}

pub fn start_input(event: Event, all_categories: [&str; 10], mut input: Input, emojis_hash: &HashMap<String, Vec<Emoji>>) -> Input {
    if let Event::Key(key_event) = event {
        match key_event.code {
            KeyCode::Down => {
                input.increment_start_point();
            },
            KeyCode::Left => {
                input.change_cur_category_left();
            },
            KeyCode::Right => {
                input.change_cur_category_right();
            },
            KeyCode::Backspace => {
                input.pop_buffer();
            },
            KeyCode::Enter => {
                //TODO!
            },
            KeyCode::Char(c) => {
                if c.is_whitespace() {
                    input.add_to_buffer(' ');
                } else if c.is_ascii_graphic() {
                    input.add_to_buffer(c);
                }
            },
            _ => {},

        }
    }

    if event == Event::Key(KeyCode::Esc.into()) {
        input.exit();
    }
    
   input 
}
