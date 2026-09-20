use ztd::kern::*;
use std::{
    path::PathBuf
};
use crate::modder::{
    PROCCESSING_EXIT_CODE
};

pub fn general_parent<Iter>(routes: &mut Iter, sholl_parent: bool) -> Option<PathBuf>
where
    Iter: Iterator<Item = PathBuf>,
{
    if sholl_parent {
        let parent = routes.next()
        .unwrap_or_bye(|bayern| {
                bayern
                .msgln("No parent was defined")
                .exit(PROCCESSING_EXIT_CODE)
        });
        return Some(parent);
    } else {
        return None;
    }
}

pub fn route_parented(parent: &mut Option<PathBuf>, route: PathBuf) -> PathBuf {
    if let Some(parent) = parent.as_mut() {
            let mut new_parent = parent.clone();
            new_parent.push(route);
    return new_parent;
    }  else {
       return route;
    }
}
