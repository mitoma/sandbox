use crate::line_generator::generate_line;
use std::io::Write;
use termion::cursor;
use termion::screen::{AlternateScreen, ToAlternateScreen, ToMainScreen};

pub(crate) struct Console {
    pub width: u16,
    pub height: u16,
    screen: AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>,
}

impl Console {
    pub fn new(
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

    pub fn to_main(&mut self) {
        self.write(&format!("{}", ToMainScreen))
    }

    pub fn to_alt(&mut self) {
        self.write(&format!("{}", ToAlternateScreen))
    }

    pub fn write_log(&mut self, line: &str, line_num: usize) {
        self.write(&self.clear_last_line_string());
        self.write(&generate_line(line.to_string(), line_num, self.height));
    }

    pub fn write(&mut self, bytes: &str) {
        self.screen.write(bytes.as_bytes()).unwrap();
    }

    pub fn clean_lastline(&mut self) {
        self.write(&self.clear_last_line_string());
    }

    pub fn enter(&mut self) {
        self.write("\n");
    }

    pub fn flush(&mut self) {
        self.screen.flush().unwrap();
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
}
