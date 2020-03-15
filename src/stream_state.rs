use crate::console::Console;
use std::collections::vec_deque::VecDeque;
use std::collections::BTreeSet;

pub(crate) struct StreamState {
    pub line_count: usize,
    log_buffer_limit: usize,
    log_buffer: VecDeque<String>,
    pub keys: BTreeSet<String>,
    pub filter_keys: Vec<String>,
    mode: Mode,
}

impl StreamState {
    pub(crate) fn new() -> StreamState {
        StreamState {
            line_count: 0,
            log_buffer_limit: 1024,
            log_buffer: VecDeque::new(),
            keys: BTreeSet::new(),
            filter_keys: Vec::new(),
            mode: Mode::TailLog,
        }
    }

    pub(crate) fn add_line(&mut self, line: &String, console: &mut Console) {
        self.line_count += 1;
        self.log_buffer.push_back(line.clone());
        if self.log_buffer.len() > self.log_buffer_limit {
            self.log_buffer.pop_front();
        }

        if let Mode::TailLog = self.mode {
            console.write_log(line, self.line_count, &self.filter_keys);
        }
    }

    pub(crate) fn rewrite_logs(&mut self, console: &mut Console) {
        let mut line_count = self.line_count;
        self.log_buffer.iter().for_each(|line| {
            line_count += 1;
            console.write_log(line, line_count, &self.filter_keys);
        });
        self.line_count = line_count;
    }

    pub(crate) fn to_tail_log_mode(&mut self) {
        self.mode = Mode::TailLog
    }

    pub(crate) fn to_key_selector_mode(&mut self) {
        self.mode = Mode::KeySelector
    }

    fn reflesh_keyset(&mut self) {
        self.keys.clear();
        let keys = self
            .log_buffer
            .iter()
            .flat_map(
                |line| match serde_json::from_str::<serde_json::Value>(&line) {
                    Ok(serde_json::Value::Object(json)) => {
                        json.keys().map(|line| line.clone()).collect()
                    }
                    _ => Vec::new(),
                },
            )
            .collect::<Vec<String>>();
        for key in keys {
            self.keys.insert(key);
        }
    }

    pub(crate) fn send_key(&mut self, console: &mut Console, c: char) {
        match self.mode {
            Mode::TailLog => match c {
                'r' => {
                    self.rewrite_logs(console);
                }
                'z' => {
                    console.to_alt();
                    self.to_key_selector_mode();
                    self.reflesh_keyset();
                    self.draw_keys(console);
                }
                _ => {}
            },
            Mode::KeySelector => match c {
                value @ '0'..='9' | value @ 'a'..='f' => {
                    let num = usize::from_str_radix(&value.to_string(), 16).unwrap();
                    self.select_key(num);
                    self.draw_keys(console);
                }
                'u' => {
                    self.unselect_all_keys();
                    self.draw_keys(console);
                }
                's' => {
                    self.select_all_keys();
                    self.draw_keys(console);
                }
                'z' => {
                    console.to_main();
                    self.to_tail_log_mode();
                    self.rewrite_logs(console);
                }
                _ => {}
            },
        }
    }

    fn unselect_all_keys(&mut self) {
        for key in self.keys.clone() {
            self.filter_keys.push(key);
        }
    }

    fn select_all_keys(&mut self) {
        self.filter_keys.clear();
    }

    fn select_key(&mut self, num: usize) {
        if let Some(selected_key) = self.keys.iter().nth(num) {
            if self.filter_keys.contains(selected_key) {
                self.filter_keys.retain(|key| key != selected_key)
            } else {
                self.filter_keys.push(selected_key.clone());
            }
        }
    }

    pub(crate) fn draw_keys(&self, console: &mut Console) {
        console.reset();

        console.write("s\t: select all\r\nu\t: unselect all\r\n0-f\t: select key\r\n\r\n");

        for (i, key) in self.keys.iter().enumerate() {
            if self.filter_keys.contains(key) {
                console.write(&format!("  {:x}:{}\r\n", i, key));
            } else {
                console.write(&format!("* {:x}:{}\r\n", i, key));
            }
        }
        console.enter()
    }
}

enum Mode {
    TailLog,
    KeySelector,
}
