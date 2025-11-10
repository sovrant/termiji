use crate::input::{self, Arrow};
use std::{collections::HashMap};
use crate::{emoji::Emoji, ui::input::Input};
use std::io::{self, stdout};
use crossterm::event::{EventStream};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::{MoveTo, Hide, Show}};
use futures::{future::FutureExt, StreamExt};
use futures_timer::Delay;
use std::time::Duration;


pub async fn start_ui(emojis_hash: HashMap<String, Vec<Emoji>>, all_categories: [&str; 10],
    mut input: Input, mut arrow: Arrow) -> Result<(), io::Error> {
    let mut reader = EventStream::new();

    loop {
        let delay = Delay::new(Duration::from_millis(10)).fuse();
        let event = reader.next().fuse();
        futures::pin_mut!(event, delay);
        
        futures::select! {
            _ = delay => { 
                arrow.set_new_end_point();

                execute!(stdout(), MoveTo(0, 0))?;
                execute!(stdout(), Clear(ClearType::All), Hide)?;
                print!("Search: {}\r", input.get_buffer());
                println!();
                print!("Category: {}\r", all_categories[arrow.get_cur_category()]);
                println!();
                
                let mut cur_cat = input.get_matched();

                if input.get_buffer().is_empty() && input.get_matched().is_empty() {
                   cur_cat = emojis_hash.get(all_categories[arrow.get_cur_category()]).unwrap();
                }

                    //FIX THIS SHIT I DONT EVEN KNOW WHAT IS WRONG
                if cur_cat.len() > arrow.get_end_point().into() {
                    for pos in arrow.get_start_point()..arrow.get_end_point().saturating_sub(3) {
                        if pos as u32 == arrow.get_cur_pos() {
                            println!("{} <==\r", cur_cat[pos as usize].get_emoji());
                        } else {
                            println!("{}\r", cur_cat[pos as usize].get_emoji());
                        }
                    }
                    //FIX THIS SHIT I DONT EVEN KNOW WHAT IS WRONG
                } else if cur_cat.len() == arrow.get_end_point() as usize {
                    #[allow(clippy::needless_range_loop)]
                    for pos in arrow.get_start_point().saturating_add(3).into()..cur_cat.len(){
                        if pos as u32 == arrow.get_cur_pos() {
                            println!("{} <==\r", cur_cat[pos].get_emoji());
                        } else {
                            println!("{}\r", cur_cat[pos].get_emoji());
                        }
                    }
                    //FIX THIS SHIT I DONT EVEN KNOW WHAT IS WRONG
                } else {
                    for (i, pos) in cur_cat.iter().enumerate() {
                        if i == arrow.get_cur_pos() as usize {
                            println!("{} <==\r", pos.get_emoji());
                        } else {
                            println!("{}\r", pos.get_emoji());
                            println!("{}\r", cur_cat.len());
                            println!("{}\r", arrow.get_end_point());
                        }
                    }
                }

            },
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        (input, arrow) = input::start_input(event, all_categories, input, &emojis_hash, arrow);

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
