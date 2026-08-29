mod attributes; use attributes::get_attributes;

use std::{
    env
};
use ztd::{
    Bayern
};

const ATTRIBUTE_EXIT_CODE: i32 = 7;
fn main() {
    let args: Vec<String> = env::args().collect(); 
    let mut bayern = Bayern::new();

    bayern.code(ATTRIBUTE_EXIT_CODE);
    let cmd = match args.get(1) {
        Some(val) => val,
        None => bayern.msgd("No instruction was given").bye()
    };

    let (action, target, sholl) = get_attributes(&mut bayern, cmd);


}


