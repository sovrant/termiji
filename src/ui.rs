use crate::input;
use std::{collections::HashMap};
use crate::{emoji::Emoji, ui::input::Input};
use std::io;
use crossterm::event::{Event, EventStream, KeyCode};
use futures::{future::FutureExt, StreamExt};
use futures_timer::Delay;
use std::time::Duration;


async fn start_ui(emojis_hash: HashMap<String, Vec<Emoji>>, all_categories: Vec<String>, mut input: Input) -> Result<(), io::Error> {
    let mut reader = EventStream::new();

    loop {
        let delay = Delay::new(Duration::from_millis(100)).fuse();
        let event = reader.next().fuse();
        futures::pin_mut!(event, delay);
        
        futures::select! {
            _ = delay => { println!("Delaying\r")},
            maybe_event = event => {
                match maybe_event {
                    Some(Ok(event)) => {
                        input = input::start_input(event, &all_categories, input, &emojis_hash);

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
