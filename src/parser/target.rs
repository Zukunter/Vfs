use crate::modder::*;
#[derive(PartialEq)]
pub enum Target {
    File,
    Directory,
    Hardlink,
    Symlink
}

const DIRECTORY_KEY: char = '/';
const SYMLINK_KEY: char = '-';
const HARDLINK_KEY: char = '=';

impl Target {
    pub fn init<AsPath: AsRef<Path>>(route: AsPath) -> (PathBuf, Self) {
        let route_ref = route.as_ref();
        
        let mut route_str = route_ref.try_to_string()
            .unwrap_or_bayern()
            .msgd(f!("The route `{route_ref:?}` has not been able to be parsed onto String"))
            .exit(PARSING_EXIT_CODE);

        let last_c = route_str.pop()
            .unwrap_or_bayern()
            .msgd(f!("The route `{route_str}` seems to be empty"))
            .exit(PARSING_EXIT_CODE);

        let target = match last_c {
            DIRECTORY_KEY => Target::Directory,
            HARDLINK_KEY => Target::Hardlink,
            SYMLINK_KEY => Target::Symlink,
            _ => return (route_ref.to_owned(), Target::File)
        };

    return (PathBuf::from(route_str), target) ; } 
}
