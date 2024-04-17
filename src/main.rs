use std::{
    collections::HashSet,
    io::{self, Read, Write},
    os::fd::AsFd,
    time::SystemTime,
};

mod build_entry;
mod parse;
mod terminal_utils;
use parse::{parse_template, preprocess_line, Template};
use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::terminal_utils::tab_complete_simple;

const DEFAULT_DATA: &str = "./data";
const DEFAULT_LOG: &str = "log";
const DEFAULT_TEMPLATES: &str = "templates";

/* NEW TEMPLATE STEPS:
 *      tokenize string
*/

/*
 * NEW LOG ENTRY STEPS:
 *      select template
*/

fn prepare_files(data: &str, log: &str, templates: &str) {}

struct LogEntry {
    name: String,
    properties: Vec<String>,
    properties_count: u8,
    time: SystemTime,
}

struct LogTemplate {
    name: String,
    template_string: String,
}

struct TabState<'a> {
    tab: usize,             // Tab state active, decrements after main loop, increments in tab
    index: usize,           // Current index in options
    pool: &'a Vec<&'a str>, // Set of acceptable entries
    options: Vec<usize>,    // List of potential incides from pool
}

fn command_log_entry(data: &str) {}

fn log_write(data: LogEntry, filename: &str) {}

fn command_template(data: String) {}

fn tab_complete() {}

/*
 * hot TODO
 * make special write actions into separate functions
 * tab complete into function (With fancy mode)
 * prevent deletion of prefix [DONE]
*/

enum CommandStates {
    Exit,
    Ok,
    Unknown,
    Clear,
}

fn process_command(message: &str) -> CommandStates {
    match message {
        "exit" | "quit" => CommandStates::Exit,
        "clear" => CommandStates::Clear,
        "log" => CommandStates::Ok,
        _ => CommandStates::Unknown,
    }
}

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();
    let mut input_string = String::new();

    let commands = (vec!["cmd", "log", "cm_d"]);
    let mut tabstate = TabState {
        tab: 0,
        index: 0,
        pool: &commands,
        options: vec![],
    };

    /*
     * Completion options will be stored in a hashmap
     * Which associates an action with a selected action
     * for example fetching another table
     */

    terminal_utils::nextline(&mut stdout, "IceLog :: ").unwrap();
    io::stdout().flush().expect("Failed to flush");

    loop {
        let input_char = stdin.next();
        if let Some(Ok(key)) = input_char {
            match key {
                termion::event::Key::Char('\n') => {
                    match process_command(input_string.as_str()) {
                        CommandStates::Exit => break,
                        CommandStates::Clear => {
                            terminal_utils::clear(&mut stdout).unwrap();
                        }
                        CommandStates::Unknown => print!(" <- Unknown Command"),
                        CommandStates::Ok => (),
                        _ => (),
                    }
                    terminal_utils::nextline(&mut stdout, "IceLog :: ").unwrap();
                    input_string.clear();
                }
                termion::event::Key::Char('\t') => {
                    tabstate.tab = 2;
                    let size = input_string.len();
                    let tab_index = 0;
                    input_string = tab_complete_simple(&mut input_string, &commands)
                        .unwrap_or(input_string.clone());

                    print!("{}", &input_string[size..input_string.len()]);
                    stdout.lock().flush().unwrap();
                }
                termion::event::Key::Backspace if input_string.len() > 0 => {
                    terminal_utils::backspace(&mut stdout).unwrap();
                    input_string.pop();
                }
                termion::event::Key::Ctrl('c') => break,
                termion::event::Key::Ctrl('l') => {
                    terminal_utils::clear(&mut stdout).unwrap();
                }
                termion::event::Key::Char(x) => {
                    terminal_utils::putchar(&mut stdout, x).unwrap();
                    input_string.push(x);
                }
                _ => (),
            }

            if tabstate.tab > 0 {
                tabstate.tab -= 1;
            }
        }
    }

    write!(stdout, "{}", termion::cursor::Left(100),).unwrap();
    stdout.lock().flush().unwrap();
    println!("");
}
