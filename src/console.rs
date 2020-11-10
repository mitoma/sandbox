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
            height,
            width,
            screen,
        }
    }

    pub(crate) fn switch_to_main(&mut self) {
        self.write(&format!("{}", ToMainScreen))
    }

    pub(crate) fn switch_to_alt(&mut self) {
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

    pub(crate) fn update_terminal_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub(crate) fn write_log(&mut self, line: &str, line_num: usize, filter_keys: &[String]) {
        self.write(&self.clear_last_line_string());
        self.write(&generate_line(
            line.to_string(),
            line_num,
            filter_keys,
            self.height,
        ));
    }

    pub(crate) fn write(&mut self, bytes: &str) {
        self.screen.write_all(bytes.as_bytes()).unwrap();
    }

    pub(crate) fn clean_lastline(&mut self) {
        self.write(&self.clear_last_line_string());
    }

    pub(crate) fn enter(&mut self) {
        self.write("\n");
    }

    pub(crate) fn cleanup(&mut self) {
        self.switch_to_main();
        self.write(&format!("{}", style::Reset));
        self.enter();
        self.flush();
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
