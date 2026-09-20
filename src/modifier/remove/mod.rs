use std::{
    path::PathBuf
};
use crate::modder::{
    helper,
    Target
};

mod directory;
mod file;

pub fn remove<IterPathBuf>(mut general_parent: Option<PathBuf>, routes: &mut IterPathBuf, sholl_force: bool) 
where 
    IterPathBuf: Iterator<Item = PathBuf> 
{
    while let Some(pre_route) =  routes.next() {
        let (route, target) = Target::init(pre_route);
        let route = helper::route_parented(&mut general_parent, route);

        if target == Target::Directory {
            directory::remove(route, sholl_force);
        } else {
            file::remove(route)
        }
    }
}
