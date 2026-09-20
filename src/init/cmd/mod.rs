use ztd::kern::*;
use crate::modder::{
    PROCCESSING_EXIT_CODE
};

pub struct CmdLower {
    pub action: String,
    pub sholl: String
}
const SPLITER_KEY: &str = "-";

impl CmdLower {
    pub fn init<Iter>(args: &mut Iter) -> Self 
    where 
        Iter: Iterator<Item = String>
    {
        let cmd = args
            .next()
            .unwrap_or_bayern()
            .msgdln("No instruction was defined")
            .exit(PROCCESSING_EXIT_CODE);
        let cmd_lower = cmd.to_ascii_lowercase();

        let mut splited_cmd_lower= cmd_lower.split(SPLITER_KEY);

        let action_lower = splited_cmd_lower
            .next()
            .unwrap_or_bayern()
            .msgdln("No `action` was defined")
            .exit(PROCCESSING_EXIT_CODE)
            .to_owned();
        let sholl_lower = splited_cmd_lower.next()
            .unwrap_or_default()
            .to_owned();

        return Self {
            action: action_lower,
            sholl: sholl_lower
        };
    }
}
