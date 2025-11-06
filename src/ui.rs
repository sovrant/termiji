use crate::input;
use std::{collections::HashMap};
use crate::{emoji::Emoji, ui::input::Input};
use std::io::{self, stdout};
use crossterm::event::{EventStream};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::{MoveTo, Hide, Show}};
use futures::{future::FutureExt, StreamExt};
use futures_timer::Delay;
use std::time::Duration;


pub async fn start_ui(emojis_hash: HashMap<String, Vec<Emoji>>, all_categories: [&str; 10], mut input: Input) -> Result<(), io::Error> {
    let mut reader = EventStream::new();

    loop {
        let delay = Delay::new(Duration::from_millis(10)).fuse();
        let event = reader.next().fuse();
        futures::pin_mut!(event, delay);
        
        futures::select! {
            _ = delay => { 
                input.set_new_end_point();
                execute!(stdout(), MoveTo(0, 0))?;
                execute!(stdout(), Clear(ClearType::All), Hide)?;
                print!("Search: {}\r", input.get_buffer());
                println!();
                print!("Category: {}\r", all_categories[input.get_cur_category()]);
                println!();
                
                let mut cur_cat: &Vec<Emoji> = emojis_hash.get(all_categories[input.get_cur_category()]).unwrap();

                if !input.get_buffer().is_empty() {
                    cur_cat = input.get_matched();
                } 

                if cur_cat.len() > input.get_end_point().into() {
                    for pos in input.get_start_point()..input.get_end_point().saturating_sub(3) {
                        println!("{}\r", cur_cat[pos as usize].get_emoji());
                    }
                } else if cur_cat.len() == input.get_end_point() as usize {
                    #[allow(clippy::needless_range_loop)]
                    for pos in input.get_start_point().into()..cur_cat.len().saturating_sub(3){
                        println!("{}\r", cur_cat[pos].get_emoji());
                    }
                } else {
                    for pos in cur_cat {
                        println!("{}\r", pos.get_emoji());
                    }
                }

            },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        input = input::start_input(event, all_categories, input, &emojis_hash);

                        if input.get_exit_status() {
                            break;
                        }
                    }
                    Some(Err(e)) => println!("Error: {e:?}\r"),
                    None => (),
                }
            }
        }
    }

    execute!(stdout(), Clear(ClearType::All), Show)?;
    Ok(())
}
