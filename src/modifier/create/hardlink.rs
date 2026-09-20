use ztd::{
    kern::*,
    vfs
};
use std::{
    path::Path
};
use crate::modder::{
    TOUCHING_EXIT_CODE
};

pub fn create<AsPath1, AsPath2>(link: AsPath1, to: AsPath2) 
where 
    AsPath1: AsRef<Path>,
    AsPath2: AsRef<Path>
{
    let link_ref = link.as_ref();
    let to_ref = to.as_ref();

    vfs::hard_link_all(link_ref, to_ref)
    .unwrap_or_bye(|bayern, err| {
        let msg_err = err.to_string();
        bayern
        .msgdln(f!("Could not crate the hardlink `{link_ref:?}`"))
        .msgdln(f!("Which shoud points to `{to_ref:?}`"))
        .msgd(f!("{msg_err}"))
        .exit(TOUCHING_EXIT_CODE)
    });
}
