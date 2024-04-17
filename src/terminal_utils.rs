use std::{
    char,
    io::{Stdout, Write},
    iter::{zip, Enumerate},
};

use anyhow::Result;
use termion::{input, raw::RawTerminal};

pub fn clear(stdout: &mut RawTerminal<Stdout>) -> Result<()> {
    write!(
        stdout,
        "{}{}\n",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
    )?;
    stdout.lock().flush()?;
    Ok(())
}

pub fn backspace(stdout: &mut RawTerminal<Stdout>) -> Result<()> {
    write!(
        stdout,
        "{}{}",
        termion::cursor::Left(1),
        termion::clear::AfterCursor,
    )?;
    stdout.lock().flush()?;
    Ok(())
}

pub fn putchar(stdout: &mut RawTerminal<Stdout>, x: char) -> Result<()> {
    write!(stdout, "{}", x)?;
    stdout.lock().flush()?;
    Ok(())
}

pub fn putstr(stdout: &mut RawTerminal<Stdout>, x: &str) -> Result<()> {
    write!(stdout, "{}", x)?;
    stdout.lock().flush()?;
    Ok(())
}

pub fn nextline(stdout: &mut RawTerminal<Stdout>, prompt: &str) -> Result<()> {
    write!(stdout, "\n{}{}", termion::cursor::Left(100), prompt)?;
    stdout.lock().flush()?;
    Ok(())
}

fn is_prefix(a: &str, b: &str) -> bool {
    if a.len() < b.len() {
        return false;
    }
    // TODO NO FUCKING CLONE WHAT THE HELL
    for (i, j) in a.to_string().chars().zip(b.to_string().chars()) {
        if i != j {
            return false;
        }
    }
    true
}

fn match_prefix(input_string: &String, options: &Vec<&str>) -> Vec<String> {
    let mut result = vec![];
    for option in options {
        if is_prefix(option, input_string) {
            result.push(option.to_string());
        }
    }
    result
}

pub fn tab_complete_simple(input_string: &String, options: &Vec<&str>) -> Option<String> {
    let viable_options = match_prefix(input_string, options);

    match viable_options.len() {
        0 => None,
        x => {
            let option = viable_options[0].to_string();
            // tab_index = (tab_index + 1) % x;
            return Some(option);
        }
    }
}
