use crate::modder::*;
use std::fs;

pub fn remove<AsPath>(route: AsPath, sholl: &Sholl)
where
    AsPath: AsRef<Path> 
{
    let route_ref = route.as_ref();

    let _result = if sholl.force {
        fs::remove_dir_all(route_ref)
    } else {
        fs::remove_dir(route_ref)
    }.unwrap_or_bye(|bayern, err|{
        let err_msg = err.to_string();
        bayern
            .msgdln(f!("Could not remove directory `{route_ref:?}`"))
            .msgdln(f!("{err_msg}"))
            .exit(TOUCHING_EXIT_CODE)
    });
}
