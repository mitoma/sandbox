use crate::console::Console;
use std::collections::vec_deque::VecDeque;
use std::collections::BTreeSet;

pub(crate) struct StreamState {
    pub line_count: usize,
    log_buffer_limit: usize,
    log_buffer: VecDeque<String>,
    mode: Mode,
}

impl StreamState {
    pub(crate) fn new() -> StreamState {
        StreamState {
            line_count: 0,
            log_buffer_limit: 1024,
            log_buffer: VecDeque::new(),
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
            console.write_log(line, self.line_count);
        }
    }

    pub(crate) fn rewrite_logs(&self, console: &mut Console) {
        self.log_buffer.iter().enumerate().for_each(|(i, line)| {
            console.write_log(line, i);
        });
    }

    // TODO: draw key list
    pub(crate) fn draw_keys(&self, console: &mut Console) {
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
            console.write(&format!("{}:{}\t", i, key));
        }
        console.enter();
    }
}

enum Mode {
    TailLog,
    KeySelector,
}
