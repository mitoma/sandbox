mod console;
mod input_receiver;
mod line_generator;
mod stream_state;

use crate::console::Console;
use crate::input_receiver::{input_receiver, StreamMessage};
use crate::stream_state::StreamState;
use std::io::{stdout, Write};
use std::time::Duration;
use termion::event::{Event, Key};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{color, cursor, style};

fn main() {
    let receiver = input_receiver();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    screen.flush().unwrap();

    let (screen_width, screen_height) = termion::terminal_size().unwrap();

    let mut console = Console::new(screen_width, screen_height, screen);

    let mut stream_state = StreamState::new();
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
            Err(_) => {
                console.write(&draw_status_line(&stream_state, &console));
            }
        }
        console.flush();
    }
}

fn draw_status_line(stream_state: &StreamState, console: &Console) -> String {
    let line = "bano | C-c: Quit, r: reload, f: filter";
    format!(
        "{}{}{}{}{}{}{}{}",
        cursor::Goto(1, console.height),
        style::Bold,
        color::Bg(color::Blue),
        color::Fg(color::White),
        line,
        std::iter::repeat(" ")
            .take(console.width as usize - line.len())
            .collect::<String>(),
        style::Reset,
        cursor::Goto(1, console.height),
    )
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
            console.clean_lastline();
            DispatchResult::Exit
        }
        Event::Key(Key::Char('\n')) => {
            console.clean_lastline();
            console.enter();
            DispatchResult::Success
        }
        Event::Key(Key::Char('r')) => {
            stream_state.rewrite_logs(console);
            DispatchResult::Success
        }
        Event::Key(Key::Char('f')) => {
            stream_state.draw_keys(console);
            DispatchResult::Success
        }
        Event::Key(Key::Char('t')) => {
            stream_state.filter_keys.push("comment".to_string());
            stream_state.filter_keys.push("country".to_string());
            DispatchResult::Success
        }
        Event::Key(Key::Char('c')) => {
            stream_state.filter_keys.clear();
            DispatchResult::Success
        }
        _ => DispatchResult::Success,
    }
}
