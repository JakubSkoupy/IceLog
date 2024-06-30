use std::{
    collections::{binary_heap::Iter, HashMap, HashSet},
    fmt::write,
    io::{self, Read, Stdin, Stdout, Write},
    os::fd::AsFd,
    time::SystemTime,
    usize,
};

mod build_entry;
mod context_table;
mod parse;
mod terminal;
use anyhow::Result;
use context_table::ContextTable;
use parse::{parse_template, preprocess_line, Template};
use terminal::MasterTerminal;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{self, raw::RawTerminal};

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

fn main() -> Result<()> {
    let stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();
    let input_string = " ".to_string();

    let prompt_literal = "IceLog ::";

    let mut master = MasterTerminal {
        output: stdout,
        input_string,
        strindex: 0,
        prompt: prompt_literal,

        map: ContextTable {
            context: 0,
            table: [&context_table::COMMANDS, &context_table::TEMPLATES],
        },

        keys: None, // Keys cache
        selected: false,
        option_index: usize::MAX,
    };
    /*
     * Completion options will be stored in a hashmap
     * Which associates an action with a selected action
     * for example fetching another table
     */

    master.nextline()?;
    //io::stdout().flush().expect("Failed to flush");

    // MAIN LOOP
    loop {
        let input_char = stdin.next();
        if let Some(Ok(key)) = input_char {
            match key {
                // Command Process
                termion::event::Key::Char('\n') => {
                    let string = master.input_string.clone();
                    println!("\nexecuting: {}", string);
                    master.nextline()?;
                }

                // Tab
                termion::event::Key::Char('\t') => {
                    master.tab_next()?;
                }

                // Backspace
                termion::event::Key::Backspace if master.input_string.len() > 1 => {
                    master.tab_complete()?;
                    master.delete(1)?;
                }

                // Writing
                termion::event::Key::Char(x) => {
                    if let Some(options) = &master.keys {
                        let option = &options[master.option_index];
                    }
                    master.tab_complete()?;
                    master.write_char(x)?;
                }

                // Signals
                termion::event::Key::Ctrl('c') => break,
                termion::event::Key::Ctrl('l') => {
                    master.clear()?;
                    master.nextline()?;
                }

                // For sake of exhaustiveness
                _ => (),
            }
        }
    }
    master.exit()?;
    Ok(())
}
