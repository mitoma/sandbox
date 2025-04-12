use crossbeam_channel::{Receiver, unbounded};
use std::io::{BufRead, Error, Stdin, stdin};
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

    // パイプの前段で peco など他のインタラクティブなコマンドを実行されていた場合に、
    // 画面描画の取り合いになることを避けるため、標準入力から最初の一行目が届くまでここで処理をブロックさせる
    if let Ok(message) = wait_first_line(&stdin) {
        stdin_sender.send(message).unwrap();
    }

    thread::spawn(move || {
        let handle = stdin.lock();
        for line in handle.lines().map_while(Result::ok) {
            stdin_sender.send(StdinStreamMessage::Text(line)).unwrap()
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

fn wait_first_line(stdin: &Stdin) -> Result<StdinStreamMessage, Error> {
    let mut first_line = String::new();
    match stdin.read_line(&mut first_line) {
        Ok(_) => Ok(StdinStreamMessage::Text(first_line)),
        Err(err) => Err(err),
    }
}
