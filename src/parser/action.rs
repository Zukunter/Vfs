use crate::modder::*;
pub enum Action {
    Create,
    Remove,
    Trash,
    Cut,
    Absolute,
    Empty
}

const CREATE_KEY: &str = "add";
const REMOVE_KEY: &str = "remove";
const TRASH_KEY: &str = "trash";
const CUT_KEY: &str = "cut";
const ABSOLUTE_KEY: &str = "absolute";
const EMPTY_KEY: &str = "empty";

impl Action {
    pub fn init<AsStr: AsRef<str>>(cmd: AsStr) -> Self {
        let cmd_ref = cmd.as_ref();
        
        match cmd_ref {
            CREATE_KEY => Action::Create,
            REMOVE_KEY => Action::Remove,
            TRASH_KEY => Action::Trash,
            CUT_KEY => Action::Cut,
            ABSOLUTE_KEY => Action::Absolute,
            EMPTY_KEY => Action::Empty,
            _ => bye_msg!(PROCCESSING_EXIT_CODE, 
                "The action `{cmd_ref}` is unknown."
            )
        }
    }
}
