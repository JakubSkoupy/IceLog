use phf::phf_map;
use std::{array, collections::HashMap};

type CmdMap<'a> = phf::Map<&'a str, usize>; // Usually function calls

pub struct ContextTable {
    pub context: usize,
    pub table: [&'static CmdMap<'static>; 2],
}

pub static COMMANDS: CmdMap = phf_map! {
    "log" => 1,
    "l" => 1,
};

pub static TEMPLATES: CmdMap = phf_map! {};

impl<'a> ContextTable {
    pub fn switch_context(&mut self, index: usize) {
        self.context = index;
    }

    pub fn get_context(&mut self) -> &CmdMap<'a> {
        self.table[self.context]
    }
}
