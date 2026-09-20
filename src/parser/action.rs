use ztd::{
    kern::*,
    bayern::Bayern
};

use crate::modder::*;
pub enum Action {
    Create,
    Remove,
    Trash,
    Cut,
    Absolute,
    Truncate,
    Empty
}

const CREATE_KEY: &str = "add";
const REMOVE_KEY: &str = "remove";
const TRUNCATE_KEY: &str = "truncate";
const TRASH_KEY: &str = "trash";
const CUT_KEY: &str = "cut";
const ABSOLUTE_KEY: &str = "absolute";
const EMPTY_KEY: &str = "empty";

impl Action {
    pub fn init<AsStr>(cmd: AsStr) -> Self 
    where 
        AsStr: AsRef<str>
    {
        let cmd_ref = cmd.as_ref();
        
        match cmd_ref {
            CREATE_KEY => Action::Create,
            REMOVE_KEY => Action::Remove,
            TRASH_KEY => Action::Trash,
            TRUNCATE_KEY => Action::Truncate,
            CUT_KEY => Action::Cut,
            ABSOLUTE_KEY => Action::Absolute,
            EMPTY_KEY => Action::Empty,
            _ => Bayern::new()
                .msgdln(f!("The action `{cmd_ref:?}` is unknown"))
                .exit(PROCCESSING_EXIT_CODE)
        }
    }
}
