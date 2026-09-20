use ztd::kern::*;
use std::{
    fs, 
    path::Path
};
use crate::modder::{
    TOUCHING_EXIT_CODE
};

pub fn create<AsPath>(route: AsPath) 
where 
    AsPath: AsRef<Path>
{
    let route_ref = route.as_ref();

    fs::create_dir_all(route_ref)
    .unwrap_or_bye(|bayern, err|{ 
        let msg_err = err.to_string();
        bayern
        .msgdln(f!("Could not create the directory `{route_ref:?}`"))
        .msgdln(f!("{msg_err}"))
        .exit(TOUCHING_EXIT_CODE)
    });
}
