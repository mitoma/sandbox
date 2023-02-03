use serde_json::Value;
use std::collections::BTreeMap;
use std::fmt::Write;
use termion::{color, cursor};

fn color(line_count: usize) -> String {
    if line_count % 2 == 0 {
        format!("{}", color::Fg(color::Cyan))
    } else {
        format!("{}", color::Fg(color::Yellow))
    }
}

pub(crate) fn generate_line(
    line: String,
    line_count: usize,
    filter_keys: &[String],
    screen_height: u16,
) -> String {
    let mut output = String::new();
    write!(
        output,
        "{}{}",
        cursor::Goto(1, screen_height),
        color(line_count)
    )
    .unwrap();

    let mut max_indent = 10;
    match serde_json::from_str::<Value>(&line) {
        Ok(Value::Object(json)) => {
            let filtered_json: BTreeMap<&String, &Value> = json
                .iter()
                .filter(|(k, _)| !filter_keys.contains(k))
                .collect();
            let indent = filtered_json
                .keys()
                .map(|k| k.chars().count())
                .max()
                .unwrap_or(0)
                + 1;
            max_indent = if indent > max_indent {
                indent
            } else {
                max_indent
            };
            filtered_json.iter().for_each(|(k, v)| {
                let parsed_string: String = match v {
                    Value::String(s) => s.to_string(),
                    Value::Bool(b) => format!("{b}"),
                    v => serde_json::to_string(v).unwrap(),
                };
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

                writeln!(
                    output,
                    "{key:>indent$} {parsed_string}{set_pos}",
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
            "{key:>indent$} {line}\n{set_pos}",
            key = "RAW",
            indent = max_indent,
            line = line,
            set_pos = cursor::Goto(1, screen_height)
        )
        .unwrap(),
    };
    output
}
