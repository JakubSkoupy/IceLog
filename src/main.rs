use std::{
    collections::{binary_heap::Iter, HashMap, HashSet},
    fmt::write,
    fs::{File, OpenOptions},
    io::{self, Read, Stdin, Stdout, Write},
    os::fd::AsFd,
    time::SystemTime,
    usize,
};

mod build_entry;
mod context_table;
mod parse;
mod terminal;
mod test_parse;
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

struct Files<'a> {
    log: &'a str,       // Append to this file
    templates: &'a str, // Templates from here
    debug_log: &'a str, // Errors
}

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
    let files = Files {
        log: "./log.ini",
        templates: "./templates.ini",
        debug_log: "./debug_log",
    };

    let stdout = io::stdout().into_raw_mode().unwrap();

    let input_string = " ".to_string();
    let prompt_literal = "IceLog ::";

    let mut master = MasterTerminal {
        output: stdout,
        input: termion::async_stdin().keys(),
        input_string,
        strindex: 0,
        prompt: prompt_literal,

        map: ContextTable {
            context: 1, // COMMANDS, NULLMAP when no completion, stuck state
            table: [
                &context_table::NULLMAP,
                &context_table::COMMANDS,
                &context_table::TEMPLATES,
            ],
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
        let input_char = master.input.next();
        if let Some(Ok(key)) = input_char {
            match key {
                // Command Process
                termion::event::Key::Char('\n') => {
                    let string = master.input_string.clone();
                    let substrings: Vec<&str> = string.split(" ").collect();
                    let cmd = substrings[1];

                    let mut argstring = String::new(); // TODO not like this obviously
                    for s in &substrings[2..] {
                        argstring.push_str(s);
                        argstring.push(' ');
                    }

                    // TODO
                    // For now, completely isolate command parsing from tab-completion
                    // Just switch the context table. There will be an array of bools
                    // for indices of the current context table. When there is only one
                    // remaining option, the context will switch automatically
                    //
                    // Potentially make this into a lib

                    // Move this into a separate function. The master terminal probably
                    // needs to be borrowed for command processing
                    match cmd {
                        "log" | "l" => {
                            master.delete(master.input_string.len().try_into()?)?;
                            let template = parse_template(argstring.clone());

                            let entry = build_entry::process_template(template, &mut master)?;
                            {
                                let mut file = OpenOptions::new()
                                    .write(true)
                                    .append(true)
                                    .open(files.log)
                                    .unwrap();
                                file.write((entry.clone() + "\n").as_bytes())?;
                            }

                            master.nextline()?;
                            println!("LOGGING: {}", entry);
                        }
                        //"cat" => {
                        //    let mut file = OpenOptions::new().read(true).open(files.log).unwrap();
                        //}
                        _ => println!("UNKNOWN COMMAND {}", cmd),
                    }

                    master.nextline()?;
                }

                // Tab
                termion::event::Key::Char('\t') => {
                    master.tab_next()?;
                }

                // Escape
                termion::event::Key::Esc => {
                    master.tab_cancel()?;
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
