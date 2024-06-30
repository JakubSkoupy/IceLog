use phf::phf_map;
use std::{array, collections::HashMap};

type CmdMap<'a> = phf::Map<&'a str, usize>; // Usually function calls

pub struct ContextTable {
    pub context: usize,
    pub table: [&'static CmdMap<'static>; 3],
}

pub static NULLMAP: CmdMap = phf_map! {}; // 0
pub static COMMANDS: CmdMap = phf_map! { // 1
    "log" => 1,
    "l" => 1,
    "test" => 0,
};
pub static TEMPLATES: CmdMap = phf_map! {}; // 2

impl<'a> ContextTable {
    pub fn switch_context(&mut self, index: usize) {
        self.context = index;
    }

    pub fn get_context(&mut self) -> &CmdMap<'a> {
        self.table[self.context]
    }
}
