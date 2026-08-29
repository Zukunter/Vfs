pub struct Sholl {
	pub tree: bool,
	pub repeat: bool,
	pub cano: bool,
	pub write: bool
}
impl Sholl {
    fn new() -> Self {
        Self {
            tree: false,
            repeat: false,
            cano: false,
            write: false
        }
    }
}

const REPEAT_CHAR: &str = "r";
const CANO_CHAR: &str = "c";
const WRITE_CHAR: &str = "w";
const TREE_CHAR: &str = "t";

pub fn get_sholl(cmd: &String) -> Sholl {
    let mut sholl = Sholl::new();

    if cmd.contains(REPEAT_CHAR) { sholl.repeat = true; }
    if cmd.contains(CANO_CHAR) { sholl.cano = true; }
    if cmd.contains(WRITE_CHAR) { sholl.write = true; }
    if cmd.contains(TREE_CHAR) { sholl.tree = true; }

return sholl ; }
