mod line_generator;

use crate::line_generator::generate_line;
use std::io::{stdin, stdout, BufRead, Write};
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::thread;
use termion::event::{Event, Key};
use termion::get_tty;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use termion::{color, cursor, style};

enum StreamMessage {
    Text(String),
    Keyboard(Event),
    TextEnd,
}

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

fn input_receiver() -> Receiver<StreamMessage> {
    let (sender, receiver) = channel();

    let tty = get_tty().unwrap();
    let stdin = stdin();

    let sender_for_stdin = sender.clone();
    thread::spawn(move || {
        let stdin = stdin.lock();
        for l in stdin.lines() {
            if let Ok(line) = l {
                sender_for_stdin.send(StreamMessage::Text(line)).unwrap();
            }
        }
        sender_for_stdin.send(StreamMessage::TextEnd).unwrap();
    });

    let tty_sender = sender;
    thread::spawn(move || {
        for e in tty.events() {
            if let Ok(evt) = e {
                tty_sender.send(StreamMessage::Keyboard(evt)).unwrap();
            }
        }
    });
    receiver
}
