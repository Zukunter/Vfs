use crate::modder::*;
use std::{
    fs, 
    path::Path
};

pub fn create<AsPath>(route: AsPath) 
where 
    AsPath: AsRef<Path>
{
    let route_ref = route.as_ref();

    fs::create_dir_all(route_ref)
    .unwrap_or_bye(|bayern, e|{ 
        bayern
            .msgdln(f!("Could not create directory `{route_ref:?}`"))
            .msgd(f!("{e:?}"))
            .exit(TOUCHING_EXIT_CODE)
    });
}
