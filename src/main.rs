mod input_receiver;
mod line_generator;

use crate::input_receiver::{input_receiver, StreamMessage};
use crate::line_generator::generate_line;
use std::collections::vec_deque::VecDeque;
use std::io::{stdout, Write};
use std::time::Duration;
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
            log_buffer_limit: 1024,
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
        match receiver.recv_timeout(Duration::from_millis(100)) {
            Ok(StreamMessage::Keyboard(evt)) => match evt {
                Event::Key(Key::Ctrl('c')) => return,
                Event::Key(Key::Char('\n')) => {
                    screen
                        .write(clean_lastline(screen_width, screen_height).as_bytes())
                        .unwrap();
                    screen.write("\n".as_bytes()).unwrap();
                }
                Event::Key(Key::Char('r')) => {
                    stream_state
                        .log_buffer
                        .iter()
                        .enumerate()
                        .for_each(|(i, line)| {
                            screen
                                .write(clean_lastline(screen_width, screen_height).as_bytes())
                                .unwrap();
                            write!(
                                screen,
                                "{}",
                                generate_line(line.to_string(), i, stream_state.screen_height)
                            )
                            .unwrap()
                        });
                    screen.flush().unwrap();
                }
                _ => {}
            },
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
                write!(screen, "{}\n", "stdio is end. quit Ctrl+C").unwrap();
            }
            Err(_) => {
                screen
                    .write(draw_status_line(&stream_state).as_bytes())
                    .unwrap();
            }
        }
        screen.flush().unwrap();
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

fn draw_status_line(stream_state: &StreamState) -> String {
    let line = "bano | C-c: Quit, r: reload, f: filter";
    format!(
        "{}{}{}{}{}{}{}{}",
        cursor::Goto(1, stream_state.screen_height),
        style::Bold,
        color::Bg(color::Blue),
        color::Fg(color::White),
        line,
        std::iter::repeat(" ")
            .take(stream_state.screen_width as usize - line.len())
            .collect::<String>(),
        style::Reset,
        cursor::Goto(1, stream_state.screen_height),
    )
}
