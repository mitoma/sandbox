mod input_receiver;
mod line_generator;

use crate::input_receiver::{input_receiver, StreamMessage};
use crate::line_generator::generate_line;
use std::collections::vec_deque::VecDeque;
use std::io::{stdout, Write};
use termion::event::{Event, Key};
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{color, cursor, style};

struct StreamState {
    line_count: usize,
    log_buffer_limit: usize,
    log_buffer: VecDeque<String>,
    screen_height: u16,
    screen_width: u16,
}

impl StreamState {
    fn new(screen_width: u16, screen_height: u16) -> StreamState {
        StreamState {
            line_count: 0,
            log_buffer_limit: 20,
            log_buffer: VecDeque::new(),
            screen_width: screen_width,
            screen_height: screen_height,
        }
    }

    fn add_line(&mut self, line: &String) {
        self.line_count += 1;
        self.log_buffer.push_back(line.clone());
        if self.log_buffer.len() > self.log_buffer_limit {
            self.log_buffer.pop_back();
        }
    }
}

fn main() {
    let receiver = input_receiver();
    let mut screen = AlternateScreen::from(stdout().into_raw_mode().unwrap());
    screen.flush().unwrap();

    let (screen_width, screen_height) = termion::terminal_size().unwrap();

    let mut stream_state = StreamState::new(screen_width, screen_height);
    loop {
        write!(
            screen,
            "{}{}{}{}{}{}{}{}",
            cursor::Goto(1, screen_height),
            style::Bold,
            color::Bg(color::Blue),
            color::Fg(color::White),
            "bano",
            std::iter::repeat(" ")
                .take((screen_width - 4) as usize)
                .collect::<String>(),
            style::Reset,
            cursor::Goto(1, screen_height),
        )
        .unwrap();
        screen.flush().unwrap();

        match receiver.recv() {
            Ok(StreamMessage::Keyboard(evt)) => {
                if evt == Event::Key(Key::Ctrl('c')) {
                    return;
                }
            }
            Ok(StreamMessage::Text(line)) => {
                screen
                    .write(clean_lastline(screen_width, screen_height).as_bytes())
                    .unwrap();
                stream_state.add_line(&line);
                write!(
                    screen,
                    "{}",
                    generate_line(line, stream_state.line_count, stream_state.screen_height)
                )
                .unwrap()
            }
            Ok(StreamMessage::TextEnd) => {
                screen
                    .write(clean_lastline(screen_width, screen_height).as_bytes())
                    .unwrap();
                write!(screen, "{}\n", "stdio is end. quit Ctrl+C").unwrap()
            }
            _ => {}
        }
    }
}

fn clean_lastline(screen_width: u16, screen_height: u16) -> String {
    format!(
        "{}{}{}",
        cursor::Goto(1, screen_height),
        std::iter::repeat(" ")
            .take(screen_width as usize)
            .collect::<String>(),
        cursor::Goto(1, screen_height)
    )
}
