use crate::modder::*;
use std::fs;

pub fn remove<AsPath>(route: AsPath) 
where 
    AsPath: AsRef<Path>
{
    let route_ref = route.as_ref();

    fs::remove_file(route_ref).unwrap_or_bye(|bayern, err|{
        let err_msg = err.to_string();
        bayern
            .msgdln(f!("Could not remove the file `{route_ref:?}`"))
            .msgdln(f!("{err_msg}"))
            .exit(TOUCHING_EXIT_CODE)
    });
}
