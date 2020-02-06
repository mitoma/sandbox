mod input_receiver;
mod line_generator;

use crate::input_receiver::{input_receiver, StreamMessage};
use crate::line_generator::generate_line;
use std::io::{stdout, Write};
use termion::event::{Event, Key};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{color, cursor, style};

fn main() {
    let receiver = input_receiver();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    screen.flush().unwrap();

    let (screen_width, screen_height) = termion::terminal_size().unwrap();

    let mut line_count = 0;
    loop {
        line_count += 1;
        write!(
            screen,
            "{}{}{}{}{}{}{}{}\n",
            cursor::Goto(1, 1),
            style::Bold,
            color::Bg(color::Blue),
            color::Fg(color::White),
            "bano",
            std::iter::repeat(" ")
                .take((screen_width - 4) as usize)
                .collect::<String>(),
            style::Reset,
            cursor::Goto(1, screen_height - 1),
        )
        .unwrap();

        match receiver.recv() {
            Ok(StreamMessage::Keyboard(evt)) => {
                if evt == Event::Key(Key::Ctrl('c')) {
                    return;
                }
            }
            Ok(StreamMessage::Text(line)) => {
                write!(screen, "{}", generate_line(line, line_count, screen_height)).unwrap()
            }
            Ok(StreamMessage::TextEnd) => {
                write!(screen, "{}\n", "stdio is end. quit Ctrl+C").unwrap()
            }
            _ => {}
        }
    }
}
