use ztd::kern::*;
use std::{
    path::{
        Path,
        PathBuf
    }
};
use crate::modder::{
    PARSING_EXIT_CODE
};

#[derive(PartialEq)]
pub enum Target {
    File,
    FileAndWrite,
    Directory,
    Hardlink,
    Symlink
}

const DIRECTORY_KEY: char = '/';
const SYMLINK_KEY: char = '-';
const HARDLINK_KEY: char = '=';
const FILE_AND_WRITE_KEY: char = ':';

impl Target {
    pub fn init<AsPath>(route: AsPath) -> (PathBuf, Self) 
    where 
        AsPath: AsRef<Path>
    {
        let route_ref = route.as_ref();
        
        let mut route_str = route_ref
            .try_to_string()
            .unwrap_or_bayern()
            .msgd(f!("The route `{route_ref:?}` has not been able to be parsed onto String"))
            .exit(PARSING_EXIT_CODE);

        let last_character = route_str
            .pop()
            .unwrap_or_bayern()
            .msgd(f!("The route `{route_str}` seems to be empty"))
            .exit(PARSING_EXIT_CODE);

        let target = match last_character {
            DIRECTORY_KEY => Target::Directory,
            HARDLINK_KEY => Target::Hardlink,
            SYMLINK_KEY => Target::Symlink,
            FILE_AND_WRITE_KEY => Target::FileAndWrite,
            _ => return (route_ref.to_owned(), Target::File)
        };

    return (PathBuf::from(route_str), target) ; } 
}
