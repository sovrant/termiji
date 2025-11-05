use crate::input;
use std::{collections::HashMap};
use crate::{emoji::Emoji, ui::input::Input};
use std::io::{self, stdout};
use crossterm::event::{EventStream};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
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
                execute!(stdout(), Clear(ClearType::All))?;
                print!("Search: {}\r", input.get_buffer());
                println!();

                let cur_cat = emojis_hash.get(all_categories[input.get_cur_category()]).unwrap();

                for pos in input.get_start_point()..input.get_end_point() - 2 {
                    println!("{}\r", cur_cat[pos as usize].get_emoji());
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

    Ok(())
}
