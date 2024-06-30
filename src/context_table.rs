use std::collections::HashMap;
type CmdMap<'a> = &'a HashMap<&'a str, usize>; // Usually function calls

pub struct ContextTable<'a> {
    context: usize,
    table: HashMap<usize, &CmdMap<'a>>,
}

static COMMANDS: CmdMap = &HashMap::from([("log", 1), ("l", 1)]);

static TABLE = ContextTable {
    context: 0,
    table: HashMap::from([(0, COMMANDS)]),
};
