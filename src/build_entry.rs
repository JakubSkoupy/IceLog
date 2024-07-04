use anyhow::Result;

use crate::{
    parse::{Field, Template, TemplateToken},
    terminal::MasterTerminal,
};
use chrono::Local;

fn timestamp() -> String {
    let date = Local::now();
    date.format("%Y/%m/%d %H:%M:%S ").to_string()
}

pub fn process_template(template: Template, terminal: &mut MasterTerminal) -> Result<String> {
    let mut result = template.name.to_string() + " ";

    for token in template.tokens {
        let value = match token {
            TemplateToken::Field(field) => match field {
                Field::Num(name, _) => process_number(terminal, &name),
                Field::Str(_name, _) => Ok("".to_string()),
            },
            TemplateToken::Text(text) => Ok(text.to_string()),
        }?;
        result.push_str(&value);
    }
    Ok(timestamp() + &result)
}

pub fn process_number(terminal: &mut MasterTerminal, name: &str) -> Result<String> {
    terminal.nextline()?;
    terminal.input_string = match name {
        "" => " Enter number: ".to_string(),
        _ => format!(" Enter {name}: ").to_string(),
    };
    terminal.write_string_complete()?;

    let mut counter = 0;
    loop {
        let input_char = terminal.input.next();
        let mut modified = true;

        if let Some(Ok(key)) = input_char {
            match key {
                termion::event::Key::Char('\n') => break,
                termion::event::Key::Char('.') => counter += 1,
                termion::event::Key::Char(',') if counter > 0 => counter -= 1,
                termion::event::Key::Backspace if counter > 0 => counter /= 10,
                termion::event::Key::Esc => break,

                // A little retarded, but acceptable I guess
                // TODO overflow protection
                termion::event::Key::Char('0') => counter = counter * 10 + 0,
                termion::event::Key::Char('1') => counter = counter * 10 + 1,
                termion::event::Key::Char('2') => counter = counter * 10 + 2,
                termion::event::Key::Char('3') => counter = counter * 10 + 3,
                termion::event::Key::Char('4') => counter = counter * 10 + 4,
                termion::event::Key::Char('5') => counter = counter * 10 + 5,
                termion::event::Key::Char('6') => counter = counter * 10 + 6,
                termion::event::Key::Char('7') => counter = counter * 10 + 7,
                termion::event::Key::Char('8') => counter = counter * 10 + 8,
                termion::event::Key::Char('9') => counter = counter * 10 + 9,
                termion::event::Key::Ctrl('c') => {
                    break;
                }
                _ => modified = false,
            }

            if modified {
                terminal.write_string_complete()?;
                print!("{}", counter);
                terminal.flush()?;
            }
        }
        //if modified {
        //    terminal.delete(digits)?;
        //
        //    for _ in (0..digits) {
        //        terminal.write_char('.')?;
        //    }
        //}
    }
    Ok(counter.to_string())
}
