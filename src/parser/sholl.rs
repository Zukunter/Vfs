pub struct Sholl {
    pub cano: bool,
    pub parent: bool,
    pub force: bool,
}

const CANO_KEY: &str = "c";
const PARENT_KEY: &str = "p";
const FORCE_KEY: &str = "f";

impl Sholl {
    pub fn init<AsStr>(cmd: AsStr) -> Self 
    where 
        AsStr: AsRef<str>
    {
        let cmd_ref = cmd.as_ref();

        Self {
            cano: cmd_ref.contains(CANO_KEY),
            parent: cmd_ref.contains(PARENT_KEY),
            force: cmd_ref.contains(FORCE_KEY),
        }
    }
}
