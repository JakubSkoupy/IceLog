use core::panic;
use std::{fmt, iter::Peekable, str::Chars, time::SystemTime};

pub enum TemplateToken {
    Text(String),
    Field(Field),
}

impl fmt::Display for TemplateToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(x) => write! {f, "{}", x},
            Self::Field(x) => write! {f, "{{{}}}", x},
        }
    }
}

pub enum Field {
    Num(String, bool),
    Str(String, bool),
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::Num(x, b) => write! {f, "{{num: {}{} }}", (if *b {".."} else {""}),x},
            Field::Str(x, b) => write! {f, "{{str: {}{} }}", (if *b {".."} else {""}),x},
        }
    }
}

pub struct Template {
    pub name: String,
    pub tokens: Vec<TemplateToken>,
    pub repeat_last: bool,
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rep = if self.repeat_last { ".." } else { "" };

        let mut string = String::new();
        for token in &self.tokens {
            string.push_str(&token.to_string());
        }

        write! {f, "{} args {} {}", self.name, string, rep}
    }
}

fn apply_repetition(line: &str, a: usize, b: usize, repetition: &str) -> String {
    let mut result = String::new();

    for _ in 1..repetition.parse().unwrap_or(1) {
        for index in a + 1..b - repetition.len() - 1 {
            result.push(line.chars().nth(index).unwrap());
        }
    }

    return result;
}

pub fn preprocess_line(line: String, mut depth: usize) -> Option<String> {
    let process = true;
    let mut line_next = line;

    while process && depth > 0 {
        let (line, process) = preprocess_line_step(line_next);
        depth -= 1;
        line_next = line;

        if !process || depth == 0 {
            return Some(remove_redundant_escapes(line_next));
        }
    }
    None
}

fn remove_redundant_escapes(line: String) -> String {
    let mut escape = false; // For escaping escape
    let mut result = "".to_string();

    for c in line.chars() {
        match c {
            '\\' if escape => {
                escape = false;
                result.push(c);
            }
            '\\' => escape = true,
            _ => result.push(c),
        }
    }
    return result;
}

fn parse_field(line_iterator: &mut Peekable<Chars>) -> TemplateToken {
    // TODO
    // range expressions
    let str_iter = line_iterator.clone();
    let match_str: String = str_iter.collect();

    let end_index = match_str.find('}').unwrap(); // If { is unmatched, panics
    let field_string: String = match_str.chars().take(end_index).collect();

    for _ in 0..=end_index {
        line_iterator.next(); // TODO not a loop
    }
    let mut words = field_string.split(':');

    // Constants nums
    let keywords_num = vec!["n", "num", "number"];
    let keywords_num_optional = vec!["n..", "num..", "number.."];

    // Constants strings
    let keywords_str = vec!["s", "str", "string"];
    let keywords_str_optional = vec!["s..", "str..", "string.."];

    let mut field = match words.next() {
        None => panic!(),
        Some(n) if keywords_num.contains(&n) => Field::Num("".to_string(), false),
        Some(n) if keywords_num_optional.contains(&n) => Field::Num("".to_string(), true),
        Some(n) if keywords_str.contains(&n) => Field::Str("".to_string(), false),
        Some(n) if keywords_str_optional.contains(&n) => Field::Str("".to_string(), true),
        x => {
            // SHOULD NEVER HAPPEN
            println!("INCORRECT: {}", x.unwrap()); // TODO debug only
            panic!();
        }
    };

    match words.next() {
        None => (),
        Some(name) => match &mut field {
            Field::Str(s, _) | Field::Num(s, _) => *s = name.to_string(),
        },
    }

    TemplateToken::Field(field)
}

pub fn parse_template(line: String) -> Template {
    let line = preprocess_line(line, 20).unwrap();

    let mut line_iterator = line.chars().peekable();
    let mut name = "[".to_string();

    match line_iterator.peek() {
        Some('"') => loop {
            match line_iterator.next() {
                Some('"') => break,
                Some(x) => name.push(x),
                None => panic!("Incorrect string"),
            }
        },
        _ => loop {
            match line_iterator.next() {
                Some(' ') => {
                    break;
                }
                Some(x) => name.push(x),
                None => panic!("Incorrect string"),
            }
        },
    }

    name.push_str("]");

    let mut result = Template {
        name,
        tokens: vec![],
        repeat_last: false,
    };

    // Tokenize
    let mut text = "".to_string();

    loop {
        match line_iterator.peek() {
            // Parse field
            Some('{') => {
                // Save previous
                result.tokens.push(TemplateToken::Text(text.clone()));
                text.clear();

                line_iterator.next();

                result.tokens.push(parse_field(&mut line_iterator));
            }
            // Text
            Some(_) => text.push(line_iterator.next().unwrap()), // TODO unwrap
            None => {
                result.tokens.push(TemplateToken::Text(text.clone()));
                break;
            }
        }
    }
    result
}

// Applies all multipliers
// TODO clean up
fn preprocess_line_step(line: String) -> (String, bool) {
    let mut level = 0; // nesting level
    let mut after_block = false; // flag for applying multiplier
    let mut continue_processing = false; // no deeper nesting

    let mut result = String::new();
    let mut stack = String::new();

    let mut rep_count_str = String::new();
    let mut escape = false;

    for c in line.chars() {
        let string_ref = match level {
            0 => &mut result,
            _ => &mut stack,
        };

        match c {
            // ----------------------- escapes
            c if escape => {
                string_ref.push(c);
                escape = false;
            }
            '\\' => {
                // string_ref.push(c);
                escape = true;
            }

            // ----------------------- nesting
            '[' => {
                level += 1;
                match level > 1 {
                    true => {
                        string_ref.push(c);
                        continue_processing = true;
                    }
                    _ => (),
                }
            }
            ']' => {
                after_block = true;
                if level > 1 {
                    string_ref.push(c);
                }
                level -= 1;
            }
            // ------------------------ space after block (Applies multiplier)
            ' ' if after_block && level == 0 => {
                for _ in 0..rep_count_str.parse().unwrap_or(1) {
                    result += stack.as_str();
                }

                rep_count_str = "".to_string();
                stack = "".to_string();

                after_block = false;
                result.push(c);
            }

            // ------------------------- multiplier (has to be after block on outmost level)
            '1'..='9' if after_block && level == 0 => rep_count_str.push(c),
            c => string_ref.push(c),
        }
    }
    (result, continue_processing)
}
