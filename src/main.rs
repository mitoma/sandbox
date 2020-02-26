mod input_receiver;
mod line_generator;

use crate::input_receiver::{input_receiver, StreamMessage};
use crate::line_generator::generate_line;
use std::collections::vec_deque::VecDeque;
use std::collections::BTreeSet;
use std::io::{stdout, Write};
use std::time::Duration;
use termion::event::{Event, Key};
use termion::raw::IntoRawMode;
use termion::screen::{AlternateScreen, ToAlternateScreen, ToMainScreen};
use termion::{color, cursor, style};

struct Console {
    width: u16,
    height: u16,
    screen: AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>,
}

impl Console {
    fn new(
        width: u16,
        height: u16,
        screen: AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>,
    ) -> Console {
        Console {
            height: height,
            width: width,
            screen: screen,
        }
    }

    fn to_main(&mut self) {
        self.write(format!("{}", ToMainScreen).as_bytes())
    }

    fn to_alt(&mut self) {
        self.write(format!("{}", ToAlternateScreen).as_bytes())
    }

    fn write_log(&mut self, line: &str, line_num: usize) {
        self.write(self.clear_last_line_string().as_bytes());
        self.write(generate_line(line.to_string(), line_num, self.height).as_bytes());
    }

    fn write(&mut self, bytes: &[u8]) {
        self.screen.write(bytes).unwrap();
    }

    fn clean_lastline(&mut self) {
        self.write(self.clear_last_line_string().as_bytes());
    }

    fn enter(&mut self) {
        self.write("\n".as_bytes());
    }

    fn clear_last_line_string(&self) -> String {
        format!(
            "{}{}{}",
            cursor::Goto(1, self.width),
            std::iter::repeat(" ")
                .take(self.width as usize)
                .collect::<String>(),
            cursor::Goto(1, self.width)
        )
    }

    fn flush(&mut self) {
        self.screen.flush().unwrap();
    }
}

struct StreamState {
    line_count: usize,
    log_buffer_limit: usize,
    log_buffer: VecDeque<String>,
}

impl StreamState {
    fn new() -> StreamState {
        StreamState {
            line_count: 0,
            log_buffer_limit: 1024,
            log_buffer: VecDeque::new(),
        }
    }

    fn add_line(&mut self, line: &String) {
        self.line_count += 1;
        self.log_buffer.push_back(line.clone());
        if self.log_buffer.len() > self.log_buffer_limit {
            self.log_buffer.pop_front();
        }
    }

    fn rewrite_logs(&self, console: &mut Console) {
        self.log_buffer.iter().enumerate().for_each(|(i, line)| {
            console.write_log(line, i);
        });
    }

    // TODO: draw key list
    fn draw_keys(&self, console: &mut Console) {
        let mut key_set: BTreeSet<String> = BTreeSet::new();
        self.log_buffer.iter().for_each(|line| {
            match serde_json::from_str::<serde_json::Value>(&line) {
                Ok(serde_json::Value::Object(json)) => {
                    for key in json.keys() {
                        key_set.insert(key.to_string());
                    }
                }
                _ => {}
            };
        });

        console.clean_lastline();
        for (i, key) in key_set.iter().enumerate() {
            console.write(format!("{}:{}\t", i, key).as_bytes());
        }
        console.write("\n".as_bytes());
    }
}

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
                stream_state.add_line(&line);
                console.clean_lastline();
                console
                    .write(generate_line(line, stream_state.line_count, console.height).as_bytes());
            }
            Ok(StreamMessage::TextEnd) => {}
            Err(_) => {
                console.write(draw_status_line(&stream_state, &console).as_bytes());
            }
        }
        console.flush();
    }
}

enum Mode {
    TailLog,
    KeySelector,
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
        Event::Key(Key::Ctrl('c')) => DispatchResult::Exit,
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
        _ => DispatchResult::Success,
    }
}
