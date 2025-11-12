use crate::input::{self, Arrow};
use std::{collections::HashMap};
use crate::{emoji::Emoji, ui::input::Input};
use std::io::{self, stdout};
use crossterm::event::{EventStream};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::{MoveTo, Hide, Show, MoveToColumn}};
use futures::{future::FutureExt, StreamExt};
use futures_timer::Delay;
use std::time::Duration;

fn print_right_side_wall(width: u16) {
    execute!(stdout(), MoveToColumn(width)).unwrap();
    print!("│");
    println!("\r")
}

fn print_header(width: u16) {
    print!("┌");
    let mut loop_skip = 0;
    for i in 0..width - 1 {
        if i <= (width / 2) - 6 || i >= (width / 2) + 5 {
            print!("─");
        } else if loop_skip == 0 {
            print!(" Termiji ");
            loop_skip = 1;
        } 
    }
    print!("┐");
}

fn print_ender(width: u16) {
    print!("└");
    for _i in 0..width - 2 {
        print!("─")
    }
    print!("┘");
    println!("\r")
}

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
                #[allow(unused)]
                let mut cur_cat = input.get_matched();

                if input.get_buffer().is_empty() && input.get_matched().is_empty() {
                   cur_cat = emojis_hash.get(all_categories[arrow.get_cur_category()]).unwrap();
                } else {
                    cur_cat = input.get_matched();
                }

                execute!(stdout(), MoveTo(0, 0))?;
                execute!(stdout(), Clear(ClearType::All), Hide)?;
                print!("Search: {}\r", input.get_buffer());
                println!();
                print!("Category: {} (Result: {})\r", all_categories[arrow.get_cur_category()], cur_cat.len());
                println!();
                print_header(arrow.get_width());
                

                if cur_cat.len() > arrow.get_end_point().into() {
                    for pos in arrow.get_start_point()..arrow.get_end_point() {
                        if pos as u32 == arrow.get_cur_pos() && input.get_copied() {
                            print!("│ {} ({}) <== (copied)", cur_cat[pos as usize].get_emoji(), cur_cat[pos as usize].get_slug());
                            print_right_side_wall(arrow.get_width());
                        } else if pos as u32 == arrow.get_cur_pos() {
                            print!("│ {} ({}) <==", cur_cat[pos as usize].get_emoji(), cur_cat[pos as usize].get_slug());
                            print_right_side_wall(arrow.get_width());
                        } else {
                            print!("│ {} ({})", cur_cat[pos as usize].get_emoji(), cur_cat[pos as usize].get_slug());
                            print_right_side_wall(arrow.get_width());
                        }
                    }
                } else if cur_cat.len() <= arrow.get_end_point() as usize {
                    #[allow(clippy::needless_range_loop)]
                    for pos in arrow.get_start_point().into()..cur_cat.len(){
                        if pos as u32 == arrow.get_cur_pos() && input.get_copied() {
                            print!("│ {} ({}) <== (copied)", cur_cat[pos].get_emoji(), cur_cat[pos].get_slug());
                            print_right_side_wall(arrow.get_width());
                        } else if pos as u32 == arrow.get_cur_pos() {
                            print!("│ {} ({}) <==", cur_cat[pos].get_emoji(), cur_cat[pos].get_slug());
                            print_right_side_wall(arrow.get_width());
                        } else {
                            print!("│ {} ({})", cur_cat[pos].get_emoji(), cur_cat[pos].get_slug());
                            print_right_side_wall(arrow.get_width());
                        }
                    }
                } 

                print_ender(arrow.get_width());
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
