use crossbeam_channel::{unbounded, Receiver};
use std::io::{stdin, BufRead};
use std::thread;
use termion::event::Event;
use termion::get_tty;
use termion::input::TermRead;

pub(crate) enum StdinStreamMessage {
    Text(String),
    TextEnd,
}

pub(crate) enum KeyStreamMessage {
    Keyboard(Event),
}

pub(crate) fn input_receiver() -> (Receiver<StdinStreamMessage>, Receiver<KeyStreamMessage>) {
    let (stdin_sender, stdin_receiver) = unbounded();
    let (key_sender, key_receiver) = unbounded();

    let stdin = stdin();
    let mut first_line = String::new();
    if stdin.read_line(&mut first_line).is_ok() {
        stdin_sender
            .send(StdinStreamMessage::Text(first_line))
            .unwrap();
    }

    thread::spawn(move || {
        let stdin = stdin.lock();
        for line in stdin.lines().flatten() {
            stdin_sender.send(StdinStreamMessage::Text(line)).unwrap();
        }
        stdin_sender.send(StdinStreamMessage::TextEnd).unwrap();
    });

    let tty = get_tty().unwrap();
    thread::spawn(move || {
        for evt in tty.events().flatten() {
            key_sender.send(KeyStreamMessage::Keyboard(evt)).unwrap();
        }
    });
    (stdin_receiver, key_receiver)
}
