use ztd::kern::*;
use std::{
    path::Path,
    fs
}; 
use crate::modder::{
    TOUCHING_EXIT_CODE
};

pub fn remove<AsPath>(route: AsPath, sholl_force: bool)
where
    AsPath: AsRef<Path> 
{
    let route_ref = route.as_ref();

    if sholl_force {
        fs::remove_dir_all(route_ref)
    } else {
        fs::remove_dir(route_ref)
    }
    .unwrap_or_bye(|bayern, err|{
        let err_msg = err.to_string();
        bayern
            .msgdln(f!("Could not remove the directory `{route_ref:?}`"))
            .msgdln(f!("{err_msg}"))
            .exit(TOUCHING_EXIT_CODE)
    })
}
