use std::{iter::Peekable, str::Chars, time::SystemTime};

fn parse_expression() {}
fn parse_text() {}

enum TemplateToken {
    Text(String),
    Field(Field),
}

enum Field {
    Num(u16, bool),
    String(String, bool),
}

struct Template {
    name: String,
    tokens: Vec<TemplateToken>,
    repeat_last: bool,
    timestamp: Option<SystemTime>,
}

fn apply_repetition(line: &str, a: usize, b: usize, repetition: &str) -> String {
    let mut result = String::new();

    for _ in (1..repetition.parse().unwrap_or(1)) {
        for index in (a + 1..b - repetition.len() - 1) {
            result.push(line.chars().nth(index).unwrap());
        }
    }

    return result;
}

pub fn preprocess_line(line: String) -> Option<String> {
    let mut process = true;
    let mut line_next = line;
    while process {
        let (line, process) = preprocess_line_step(line_next);
        line_next = line;

        if !process {
            return Some(line_next);
        }
    }
    None
}
// Applies all multipliers
pub fn preprocess_line_step(line: String) -> (String, bool) {
    let mut index = None; // Stores indexes of '['
    let mut level = 0;
    let mut after_block = false;
    let mut continue_processing = false;

    let mut result = String::new();
    let mut stack = String::new
    let mut rep_count_str = String::new();
    let mut escape = false;

    for (i, c) in line.chars().enumerate() {
        match c {
            c if escape => {
                result.push(c);
                escape = false;
            }
            '\\' => escape = true,
            '[' => {
                level += 1;

                match level > 1 {
                    true => {
                        result.push(c);
                        continue_processing = true;
                    }
                    false => index = Some(i),
                }
            }
            ']' => {
                after_block = true;

                match level > 1 {
                    true => result.push(c),
                    _ => (),
                }

                level -= 1;
            }
            ' ' if after_block && level == 0 => {
                let nested =
                    apply_repetition(line.as_str(), index.unwrap(), i, rep_count_str.as_str());

                result += nested.as_str();

                rep_count_str = "".to_string();

                after_block = false;
                index = None;
                result.push(c);
            }

            '1'..='9' if after_block && level == 0 => rep_count_str.push(c), // APPLY MULT (MAX 9 I guess)
            // c if after_block && level == 1 => panic!(),                      // expected mult
            c => result.push(c),
        }
    }

    println!("{}", result);
    (result, continue_processing)
}
