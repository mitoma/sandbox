use serde_json::Value;
use std::fmt::Write;
use termion::{color, cursor};

fn color(line_count: u16) -> String {
    if line_count % 2 == 0 {
        format!("{}", color::Fg(color::Magenta))
    } else {
        format!("{}", color::Fg(color::Yellow))
    }
}

pub(crate) fn generate_line(line: String, line_count: u16, screen_height: u16) -> String {
    let mut output = String::from("");
    write!(
        output,
        "{}{}",
        cursor::Goto(1, screen_height),
        color(line_count)
    )
    .unwrap();

    match serde_json::from_str::<Value>(&line) {
        Ok(Value::Object(json)) => json.iter().for_each(|(k, v)| {
            let parsed_string: String = serde_json::to_string(v).unwrap();
            write!(
                output,
                "{}\t{}{}\n",
                k,
                parsed_string,
                cursor::Goto(1, screen_height)
            )
            .unwrap();
        }),
        _ => write!(output, "RAW\t{}\n", line).unwrap(),
    };
    output
}
