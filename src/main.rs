use std::io::{stdin, stdout, BufRead};
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{thread, time};
use termion::cursor::DetectCursorPos;
use termion::event::{Event, Key};
use termion::get_tty;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, cursor, style};

struct CursorPosition {
    x: u16,
    y: u16,
}

fn main() {
    let (tail_sender, tail_receiver) = channel();
    let (tty_sender, tty_receiver) = channel();

    let tty = get_tty().unwrap();
    let stdin = stdin();

    thread::spawn(move || {
        let stdin = stdin.lock();
        for l in stdin.lines() {
            if let Ok(line) = l {
                tail_sender.send(line).unwrap();
            }
        }
    });

    thread::spawn(move || {
        for e in tty.events() {
            if let Ok(evt) = e {
                tty_sender.send(evt).unwrap();
            }
        }
    });

    let mut stdout = stdout().into_raw_mode().unwrap();

    let (screen_width, screen_height) = termion::terminal_size().unwrap();
    //    let (x, y) = stdout.cursor_pos().unwrap();

    loop {
        if let Ok(evt) = tty_receiver.recv_timeout(Duration::from_millis(5)) {
            if evt == Event::Key(Key::Ctrl('c')) {
                return;
            }
            println!("{:?}", evt);
        }

        if let Ok(line) = tail_receiver.recv_timeout(Duration::from_millis(5)) {
            println!(
                "{}Hello.{}{}{}{}{}",
                cursor::Goto(1, 1),
                color::Fg(color::Yellow),
                style::Reset,
                cursor::Goto(1, screen_height),
                line,
                cursor::Goto(1, screen_height),
            );
        }
    }
}
