use std::path::Path;
use ztd::vfs;
use crate::modder::*;

pub fn create<AsPath1, AsPath2>(link: AsPath1, to: AsPath2) 
where 
    AsPath1: AsRef<Path>,
    AsPath2: AsRef<Path>
{
    let link_ref = link.as_ref();
    let to_ref = to.as_ref();

    vfs::hard_link_all(link_ref, to_ref)
    .unwrap_or_bye(|bayern, e| {
        bayern
            .msgdln(f!("Could not crate the hardlink `{link_ref:?}`, which points to `{to_ref:?}`"))
            .msgd(f!("{e:?}"))
            .exit(TOUCHING_EXIT_CODE)
    });
}
