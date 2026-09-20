use ztd::{
    kern::*,
    cfg,
    vfs
};
use std::{
    path::Path
};
use crate::modder::{
    Target,
    TOUCHING_EXIT_CODE
};

pub fn create<AsPath1, AsPath2>(link: AsPath1, to: AsPath2) 
where 
    AsPath1: AsRef<Path>,
    AsPath2: AsRef<Path>        
{
    let link_ref = link.as_ref();
    let to_ref = to.as_ref();

    cfg::windows(|| {
        let (to_real_route, to_kind_of_target) = Target::init(to_ref);
        let is_dir = to_kind_of_target == Target::Directory;

        vfs::create_symlink_all!(link_ref, to_real_route, is_dir)
    }).otherwisse(|| {
        vfs::create_symlink_all!(link_ref, to_ref)
    })
    .unwrap_or_bye(|bayern, err| {
        let msg_err = err.to_string();
        bayern
        .msgdln(f!("Could no create the symlink `{link_ref:?}`, which points to `{to_ref:?}`"))
        .msgd(f!("{msg_err}"))
        .exit(TOUCHING_EXIT_CODE)
    });
}
