pub struct Sholl {
    pub cano: bool,
    pub parent: bool,
    pub force: bool,
    pub write: bool
}

const CANO_KEY: &str = "c";
const PARENT_KEY: &str = "p";
const FORCE_KEY: &str = "f";
const WRITE_KEY: &str = "w";

impl Sholl {
    pub fn init<AsStr: AsRef<str>>(cmd: AsStr) -> Self {
        let cmd_ref = cmd.as_ref();

        Self {
            cano: cmd_ref.contains(CANO_KEY),
            parent: cmd_ref.contains(PARENT_KEY),
            force: cmd_ref.contains(FORCE_KEY),
            write: cmd_ref.contains(WRITE_KEY)
        }
    }
}
