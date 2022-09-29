mod console;
mod input_receiver;
mod line_generator;
mod stream_state;

use crate::console::Console;
use crate::input_receiver::{input_receiver, StdinStreamMessage};
use crate::stream_state::{StreamState, WithMetaKey};
use clap::Parser;
use crossbeam_channel::select;
use input_receiver::KeyStreamMessage;
use nix::sys::signal::{killpg, SIGTERM};
use nix::unistd::Pid;
use std::io::{stdout, Write};
use std::time::Duration;
use termion::event::{Event, Key};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;

#[derive(Parser, Debug)]
#[command(author, version, about = "cli viewer of json log stream", long_about = None)]
struct Args {
    // column names
    #[arg(short, long, default_value = "short")]
    excludes: Vec<String>,
}

fn main() {
    let args = Args::parse();

    let (stdin_receiver, key_receiver) = input_receiver();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    screen.flush().unwrap();

    let (screen_width, screen_height) = termion::terminal_size().unwrap();

    let mut console = Console::new(screen_width, screen_height, screen);

    let mut stream_state = StreamState::new();

    for exclude_column in args.excludes {
        stream_state.filter_keys.push(exclude_column.to_string());
    }

    console.switch_to_main();
    loop {
        let (screen_width, screen_height) = termion::terminal_size().unwrap();
        console.update_terminal_size(screen_width, screen_height);

        select! {
            recv(key_receiver) -> msg => {
                match msg {
                    Ok(KeyStreamMessage::Keyboard(evt)) => {
                        match dispatch_keyevent(evt, &mut stream_state, &mut console) {
                            DispatchResult::Success => {}
                            DispatchResult::Exit => break
                        };
                    },
                    Err(crossbeam_channel::RecvError) => {}
                }
            },
            recv(stdin_receiver) -> msg => {
                match msg {
                    Ok(StdinStreamMessage::Text(line)) => {
                        stream_state.add_line(&line, &mut console);
                    }
                    Ok(StdinStreamMessage::TextEnd) => {},
                    Err(crossbeam_channel::RecvError) => {}
                }
            },
            default(Duration::from_secs(10)) => continue,
        };
        console.flush();
    }
    console.switch_to_main();
    console.flush();
    kill_pg();
}

fn kill_pg() {
    if let Err(errno) = killpg(Pid::from_raw(0), SIGTERM) {
        println!("{}", errno.desc());
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
