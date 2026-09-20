use ztd::kern::*;
use std::{
    path::PathBuf,
    fs
};
use crate::modder::{
    helper,
    PARSING_EXIT_CODE
};

pub fn absolute<IterPathBuf>(mut general_parent: Option<PathBuf>, routes: &mut IterPathBuf) 
where 
    IterPathBuf: Iterator<Item = PathBuf>
{
    while let Some(route) = routes.next() {
        let route = helper::route_parented(&mut general_parent, route);

        let cano_route = fs::canonicalize(&route)
        .unwrap_or_bye(|bayern, err|{
            let msg_err = err.to_string();            
            bayern
                .msgdln(f!("Could not canonicalize the route `{route:?}`"))
                .msgdln(f!("{msg_err}"))
                .exit(PARSING_EXIT_CODE)
        });

        println!("{route:?} -> {cano_route:?}");
    }
}
