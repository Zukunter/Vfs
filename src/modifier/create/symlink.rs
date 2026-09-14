use crate::modder::*;
use std::path::Path;
use ztd::vfs;

pub fn create<AsPath1, AsPath2>(link: AsPath1, to: AsPath2) 
where 
    AsPath1: AsRef<Path>,
    AsPath2: AsRef<Path>        
{
    let link_ref = link.as_ref();
    let to_ref = to.as_ref();

    vfs::create_symlink_all!(link_ref, to_ref)
    .unwrap_or_bye(|bayern, e| {
        bayern
            .msgdln(f!("Could no create the symlink `{link_ref:?}`, which points to `{to_ref:?}`"))
            .msgd(f!("{e:?}"))
            .exit(TOUCHING_EXIT_CODE)
    });
}
