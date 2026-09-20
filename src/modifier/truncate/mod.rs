use ztd::{
    kern::*,
    bayern::Bayern
};
use std::{
    fs::File, 
    path::PathBuf
};
use crate::modder::{
    PROCCESSING_EXIT_CODE,
    TOUCHING_EXIT_CODE,
    PARSING_EXIT_CODE,
    helper
};

pub fn truncate<IterPathBuf>(mut general_parent: Option<PathBuf>, routes: &mut IterPathBuf)
where 
    IterPathBuf: Iterator<Item = PathBuf>
{
    while let Some(route) = routes.next() {
        let route = helper::route_parented(&mut general_parent, route);

        if !route.is_file() {
            Bayern::new()
            .msgdln(f!("The route `{route:?}` is not a file, thus is not possible to truncate it"))
            .exit(PROCCESSING_EXIT_CODE)
        }

        let size_string = routes.next()
            .unwrap_or_bayern()
            .msgdln(f!("No size to truncate file `{route:?}` was assigned"))
            .exit(PROCCESSING_EXIT_CODE)
            .try_to_string()
            .unwrap_or_bye(|bayern, os_str| {
                bayern
                .msgdln(f!("Could not parse the OsStr `{os_str:?}` onto string"))
                .exit(PARSING_EXIT_CODE)
            });

        let size = size_string.parse::<u64>()
        .unwrap_or_bye(|bayern, err| {
            let msg_err = err.to_string();
            bayern
            .msgdln(f!("Could not parse the String `{size_string}` onto an u64"))
            .msgdln(f!("{msg_err}"))
            .exit(PARSING_EXIT_CODE)
        });

        let file_writer = File::create(&route)
        .unwrap_or_bye(|bayern, err| {
            let msg_err = err.to_string();
            bayern
            .msgdln(f!("Could not open the file `{route:?}` while trying to truncate it"))
            .msgdln(f!("{msg_err}"))
            .exit(TOUCHING_EXIT_CODE)
        });

        file_writer.set_len(size)
        .unwrap_or_bye(|bayern, err| {
            let msg_err = err.to_string();
            bayern
            .msgdln(f!("Could not truncate the file `{route:?}`"))
            .msgdln(f!("{msg_err}"))
            .exit(TOUCHING_EXIT_CODE)
        });

    }
}
