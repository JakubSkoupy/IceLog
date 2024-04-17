use anyhow::Result;
use std::{
    collections::HashSet,
    io::{self, Read, Stdin, Stdout, Write},
    os::fd::AsFd,
    time::SystemTime,
};
use termion::input::TermRead;
use termion::{input, raw::RawTerminal};

use crate::parse::{Field, Template, TemplateToken};

pub fn process_template(template: Template) {
    let mut result = "".to_string();

    for token in template.tokens {
        match token {
            TemplateToken::Field(field) => match field {
                Field::Num(name, _) => (),
                _ => (),
            },
            Text => (),
            _ => (),
        }
    }
    ()
}

pub fn process_number(stdout: &mut RawTerminal<Stdout>) -> Result<()> {
    let mut stdin = termion::async_stdin().keys();

    let mut counter = 0;
    loop {
        let input_char = stdin.next();
        if let Some(Ok(key)) = input_char {
            match key {
                termion::event::Key::Char('\n') => break,
                termion::event::Key::Char('.') => counter += 1,
                termion::event::Key::Char(',') if counter > 0 => counter -= 1,
                termion::event::Key::Backspace if counter > 0 => counter /= 10,

                // A little retarded, but acceptable I guess
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
                _ => (),
            }
        }

        let digits = counter;
        write!(
            stdout,
            "{}{}",
            termion::cursor::Left(1),
            termion::clear::AfterCursor,
        )?;
        stdout.lock().flush()?;
    }
    Ok(())
}
