use crate::line_generator::generate_line;
use std::io::Write;
use termion::cursor;
use termion::screen::{AlternateScreen, ToAlternateScreen, ToMainScreen};
use termion::{clear, style};

pub(crate) struct Console {
    pub width: u16,
    pub height: u16,
    screen: AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>,
}

impl Console {
    pub(crate) fn new(
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

    pub(crate) fn to_main(&mut self) {
        self.write(&format!("{}", ToMainScreen))
    }

    pub(crate) fn to_alt(&mut self) {
        self.write(&format!("{}", ToAlternateScreen))
    }

    pub(crate) fn reset(&mut self) {
        self.write(&format!(
            "{}{}{}",
            clear::All,
            cursor::Goto(1, 1),
            style::Reset
        ))
    }

    pub(crate) fn write_log(&mut self, line: &str, line_num: usize, filter_keys: &Vec<String>) {
        self.write(&self.clear_last_line_string());
        self.write(&generate_line(
            line.to_string(),
            line_num,
            filter_keys,
            self.height,
        ));
    }

    pub(crate) fn write(&mut self, bytes: &str) {
        self.screen.write(bytes.as_bytes()).unwrap();
    }

    pub(crate) fn clean_lastline(&mut self) {
        self.write(&self.clear_last_line_string());
    }

    pub(crate) fn enter(&mut self) {
        self.write("\n");
    }

    pub(crate) fn flush(&mut self) {
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
