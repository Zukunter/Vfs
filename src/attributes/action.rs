use ztd::Bayern;

const CREATE_CHAR: &str = "+"; const CREATE_POS: u8 = 0;
const NEW_CHAR: &str = "++"; const NEW_POS: u8 = 1;
const REMOVE_CHAR: &str = "-"; const REMOVE_POS: u8 = 2;
const DESTROY_CHAR: &str = "--"; const DESTROY_POS: u8 = 3;

pub fn get_action(bayern: &mut Bayern, cmd: &String) ->  u8 {
    if cmd.contains(NEW_CHAR) { NEW_POS }
    else if cmd.contains(DESTROY_CHAR) { DESTROY_POS }
    else if cmd.contains(CREATE_CHAR) { CREATE_POS }
    else if cmd.contains(REMOVE_CHAR) { REMOVE_POS }
    else { bayern.msgd("No action was defined").bye(); }
}




