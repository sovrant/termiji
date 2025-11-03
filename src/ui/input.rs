use crossterm::event::{Event, KeyCode};
use std::io;
use terminal_size::{Width, Height, terminal_size};

pub struct Input {
    buffer: String,
    cur_position: u32,
    start_point: u16,
    end_point: u16,
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
        self.start_point.saturating_add(1);
    }

    pub fn decrement_start_point(&mut self) {
        self.start_point.saturating_sub(1); 
    }

    pub fn get_start_point(&self) -> u16 {
        self.start_point
    }

    pub fn get_end_point(&self) -> u16 {
        self.end_point
    }

    pub fn set_new_end_point(&mut self) {
        let size = terminal_size();
        if let Some((Width(_w), Height(h))) = size && h != self.end_point {
            self.end_point = h;
        }
    }
}

pub fn start_input(event: Event, mut input: Input) -> Input {
    if event == Event::Key(KeyCode::).into()) {
        println!("you've pressed a special letter \"e\"\r");
    } else if let Event::Key(key_event) = event {
        
    }

    if event == Event::Key(KeyCode::Esc.into()) {
        input.exit();
    }
    
   input 
}
