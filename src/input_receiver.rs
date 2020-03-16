use std::io::{stdin, BufRead};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use termion::event::Event;
use termion::get_tty;
use termion::input::TermRead;

pub(crate) enum StreamMessage {
    Text(String),
    Keyboard(Event),
    TextEnd,
}

pub(crate) fn input_receiver() -> Receiver<StreamMessage> {
    let (sender, receiver) = channel();

    let stdin = stdin();
    let mut first_line = String::new();
    if let Ok(_) = stdin.read_line(&mut first_line) {
        sender.send(StreamMessage::Text(first_line)).unwrap();
    }

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

    let tty = get_tty().unwrap();
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
