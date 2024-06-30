use anyhow::Result;
use std::cmp;
use std::{
    char,
    collections::HashMap,
    io::{Stdout, Write},
};
use termion::{self, raw::RawTerminal};

type CmdMap<'a> = &'a HashMap<&'a str, usize>; // Usually function calls

pub struct MasterTerminal<'a> {
    pub output: RawTerminal<Stdout>,
    pub input_string: String,
    pub strindex: u16, // Cursor position
    pub prompt: &'static str,

    // TAB fields
    pub map: Option<CmdMap<'a>>,
    pub option_index: usize,
    pub selected: bool,            // Selected option (useful for commands esp log)
    pub keys: Option<Vec<String>>, // TODO Iterator
}

impl<'a> MasterTerminal<'a> {
    fn clear_string(&mut self) {
        self.input_string.clear();
        self.input_string.push_str(" ");
    }

    fn flush(&mut self) -> Result<()> {
        self.output.lock().flush()?;
        Ok(())
    }

    fn move_cursor(&mut self, count: i32) -> Result<()> {
        let abs_count: u16 = count.abs().try_into()?;
        match count {
            (0..) => {
                write!(self.output, "{}", termion::cursor::Right(abs_count))?;
            }
            _ => {
                write!(self.output, "{}", termion::cursor::Left(abs_count))?;
            }
        };
        self.output.lock().flush()?;
        Ok(())
    }

    pub fn clear(&mut self) -> Result<()> {
        write!(
            self.output,
            "{}{}\n",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
        )?;
        self.flush()?;
        self.clear_string();
        Ok(())
    }

    pub fn delete(&mut self, count: u16) -> Result<()> {
        let length = self.input_string.len();
        self.keys = None;

        match count {
            // < because of one space padding
            (0..) if count < length.try_into()? => {
                self.input_string.truncate(length - count as usize);

                // -1 because index vs length
                self.strindex = cmp::min(self.strindex, self.input_string.len() as u16 - 1);
                self.write_string_complete()?;
            }
            _ => (),
        }
        Ok(())
    }

    fn write_string_complete(&mut self) -> Result<()> {
        write!(
            self.output,
            "{}{}{}{}",
            termion::cursor::Left(256),
            termion::clear::AfterCursor,
            self.prompt,
            self.input_string,
        )?;
        self.output.lock().flush()?;

        Ok(())
    }

    pub fn write_char(&mut self, x: char) -> Result<()> {
        self.keys = None;
        self.input_string.push(x);
        self.strindex += 1;
        self.write_string_complete()?;
        Ok(())
    }

    pub fn nextline(&mut self) -> Result<()> {
        let _string = self.input_string.clone();
        self.clear_string();
        write!(
            self.output,
            "\n{}{}{}",
            termion::cursor::Left(100),
            self.prompt,
            self.input_string,
        )?;
        self.strindex = 0;
        self.flush()?;

        //println!("DEBUG: {}", string);
        Ok(())
    }

    pub fn tab_complete(&mut self) -> Result<()> {
        match &self.keys {
            None => (),
            Some(keys) => {
                let option = keys[self.option_index].to_string();
                self.strindex += option.len() as u16; // TODO error control
                self.move_cursor(option.len() as i32)?; // TODO same

                self.keys = None;
                self.write_string_complete()?;
            }
        }
        Ok(())
    }

    pub fn tab_next(&mut self) -> Result<()> {
        // Shorten back to str index
        self.input_string.truncate(self.strindex as usize + 1);
        let prefix = self.input_string.split(" ").last();

        match &self.map {
            None => return Ok(()),
            Some(map) => match &self.keys {
                None => {
                    let options: Vec<&&str> = map.keys().collect();

                    match prefix {
                        None => (),
                        Some(x) => {
                            let keys = match_prefix(x, options.as_ref());
                            if keys.len() > 0 {
                                self.keys = Some(keys);
                            }
                        }
                    }
                }
                Some(_keys) => (),
            },
        }
        let index = self.strindex;

        if let Some(keys) = &self.keys {
            self.option_index = match self.option_index {
                usize::MAX => 0,
                x => (x + 1) % keys.len(),
            };

            let option = keys[self.option_index].clone()[prefix.unwrap().len()..].to_string();

            self.input_string.push_str(&option);
            self.write_string_complete()?;

            self.strindex = index;
            self.move_cursor(-option.len().try_into()?)?;
        }

        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        write!(self.output, "{}", termion::cursor::Left(100),).unwrap();
        self.output.lock().flush().unwrap();
        println!("");
        Ok(())
    }
}

fn is_prefix(a: &str, b: &str) -> bool {
    if a.len() < b.len() {
        return false;
    }
    // TODO NO CLONING
    for (i, j) in a.to_string().chars().zip(b.to_string().chars()) {
        if i != j {
            return false;
        }
    }
    true
}

fn match_prefix(x: &str, options: &Vec<&&str>) -> Vec<String> {
    let mut result = vec![];

    for option in options {
        if is_prefix(option, x) {
            result.push(option.to_string());
        }
    }

    result
}
