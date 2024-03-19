use std::time::SystemTime;

mod parse;
use parse::preprocess_line;

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

fn command_log_entry(data: &str) {}

fn log_write(data: LogEntry, filename: &str) {}

fn command_template(data: String) {}

fn main() {
    // prepare_files(DEFAULT_DATA, DEFAULT_LOG, DEFAULT_TEMPLATES);
    let arbitrary_line = "This line [ Recursively repeats [ \\[this shit \\] ]2 ]3 \n".to_string();

    let result_line = preprocess_line(arbitrary_line.clone()).unwrap();
    println!("SOURCE: {}\n\n", arbitrary_line);
    println!("{}", result_line);
    // loop {}
}
