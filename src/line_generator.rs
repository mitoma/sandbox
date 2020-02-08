use serde_json::Value;
use std::fmt::Write;
use termion::{color, cursor, style};

fn color(line_count: usize) -> String {
    if line_count % 2 == 0 {
        format!("{}", color::Fg(color::Magenta))
    } else {
        format!("{}", color::Fg(color::Yellow))
    }
}

pub(crate) fn generate_line(line: String, line_count: usize, screen_height: u16) -> String {
    let mut output = String::new();
    write!(
        output,
        "{}{}",
        cursor::Goto(1, screen_height),
        color(line_count)
    )
    .unwrap();

    // TODO 仮おき
    let mut max_indent = 10;
    match serde_json::from_str::<Value>(&line) {
        Ok(Value::Object(json)) => {
            let indent = json.keys().map(|k| k.len()).max().unwrap_or(0) + 1;
            max_indent = if indent > max_indent {
                indent
            } else {
                max_indent
            };
            json.iter().for_each(|(k, v)| {
                let parsed_string: String = match v {
                    Value::String(s) => s.to_string(),
                    Value::Bool(b) => format!("{}", b),
                    v => serde_json::to_string(v).unwrap(),
                };
                //            let parsed_string: String = serde_json::to_string(v).unwrap();
                let ref_parsed: &str = &parsed_string;
                let vec: Vec<&str> = ref_parsed.split('\n').collect();
                let joined = vec.join(
                    format!(
                        "\n{head}{space:>indent$} ",
                        head = cursor::Goto(1, screen_height),
                        space = "",
                        indent = max_indent
                    )
                    .as_ref(),
                );

                write!(
                    output,
                    "{key:>indent$} {parsed_string}{set_pos}\n",
                    key = k,
                    indent = max_indent,
                    parsed_string = joined,
                    set_pos = cursor::Goto(1, screen_height)
                )
                .unwrap();
            })
        }
        _ => write!(
            output,
            "{key:>indent$} {line}\n",
            key = "RAW",
            indent = max_indent,
            line = line
        )
        .unwrap(),
    };
    output
}
