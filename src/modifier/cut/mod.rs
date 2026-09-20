use ztd::kern::*;
use std::{
    path::PathBuf,
    fs
};
use crate::modder::{
    helper,
    TOUCHING_EXIT_CODE
};

pub fn cut<IterPathBuf>(mut general_parent: Option<PathBuf>, routes: &mut IterPathBuf) 
where 
    IterPathBuf: Iterator<Item = PathBuf>
{
    while let Some(pre_route) = routes.next() {
        let route = helper::route_parented(&mut general_parent, pre_route);
        let new_name = routes
            .next()
            .unwrap_or_bayern()
            .msgdln("No new name was sent")
            .exit(TOUCHING_EXIT_CODE);

        fs::rename(&route, &new_name)
        .unwrap_or_bye(|bayern, err|{
            let msg_err = err.to_string();
            bayern
                .msgdln(f!("Could not rename the entry `{route:?}` as `{new_name:?}`"))
                .msgdln(f!("{msg_err}"))
                .exit(TOUCHING_EXIT_CODE)
        });
    }
}
