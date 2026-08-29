use ztd::Bayern;

const FILE_CHAR: &str = "f"; const FILE_POS: u8 = 0;
const DIR_CHAR: &str = "d"; const DIR_POS: u8 = 1;
const SYMLINK_CHAR: &str = "s"; const SYMLINK_POS: u8 = 2;
const HARDLINK_CHAR: &str = "h"; const HARDLINK_POS: u8 = 3;
const INDETERMINATED_CHAR: &str = "i"; const INDETERMINATED_POS: u8 = 4;

const FILE_AND_DIR_POS: u8 = 5;

pub fn get_target(bayern: &mut Bayern, cmd: &String) -> u8 {
    if cmd.contains(FILE_CHAR) && cmd.contains(DIR_CHAR) { FILE_AND_DIR_POS }
    else if cmd.contains(FILE_CHAR) { FILE_POS }
    else if cmd.contains(DIR_CHAR) { DIR_POS }
    else if cmd.contains(SYMLINK_CHAR) { SYMLINK_POS }
    else if cmd.contains(HARDLINK_CHAR) { HARDLINK_POS }
    else if cmd.contains(INDETERMINATED_CHAR) { INDETERMINATED_POS }
    else { bayern.msgd("No target was defined").bye(); }
}
