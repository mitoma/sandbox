mod console;
mod input_receiver;
mod line_generator;
mod stream_state;

#[macro_use]
extern crate clap;

use crate::console::Console;
use crate::input_receiver::{input_receiver, StreamMessage};
use crate::stream_state::{StreamState, WithMetaKey};
use clap::Arg;
use std::io::{stdout, Write};
use std::time::Duration;
use termion::event::{Event, Key};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

fn main() {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("exclude")
                .multiple(true)
                .short("e")
                .long("exclude")
                .value_name("column name")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let receiver = input_receiver();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    screen.flush().unwrap();

    let (screen_width, screen_height) = termion::terminal_size().unwrap();

    let mut console = Console::new(screen_width, screen_height, screen);

    let mut stream_state = StreamState::new();

    if let Some(exclude_columns) = matches.values_of("exclude") {
        for exclude_column in exclude_columns {
            stream_state.filter_keys.push(exclude_column.to_string());
        }
    }

    console.to_main();
    loop {
        match receiver.recv_timeout(Duration::from_millis(100)) {
            Ok(StreamMessage::Keyboard(evt)) => {
                match dispatch_keyevent(evt, &mut stream_state, &mut console) {
                    DispatchResult::Success => {}
                    DispatchResult::Exit => return,
                };
            }
            Ok(StreamMessage::Text(line)) => {
                stream_state.add_line(&line, &mut console);
            }
            Ok(StreamMessage::TextEnd) => {}
            Err(_) => {}
        }
        console.flush();
    }
}

enum DispatchResult {
    Success,
    Exit,
}

fn dispatch_keyevent(
    evt: Event,
    stream_state: &mut StreamState,
    console: &mut Console,
) -> DispatchResult {
    match evt {
        Event::Key(Key::Ctrl('c')) => {
            console.cleanup();
            DispatchResult::Exit
        }
        Event::Key(Key::Char('\n')) => {
            console.clean_lastline();
            console.enter();
            DispatchResult::Success
        }
        Event::Key(Key::Char(c)) => {
            stream_state.send_key(console, c, WithMetaKey::None);
            DispatchResult::Success
        }
        Event::Key(Key::Alt(c)) => {
            stream_state.send_key(console, c, WithMetaKey::Alt);
            DispatchResult::Success
        }
        _ => DispatchResult::Success,
    }
}
