use ztd::kern::*;
use std::{
    path::PathBuf
};
use crate::modder::{
    helper,
    PROCCESSING_EXIT_CODE,
    Target
};
mod file; 
mod directory;
mod symlink;
mod hardlink;

pub fn create<IterPathBuf>(mut general_parent: Option<PathBuf>, routes: &mut IterPathBuf, sholl_force: bool) 
where 
    IterPathBuf: Iterator<Item = PathBuf>
{
    while let Some(pre_route) = routes.next() {
        let (route, target) = Target::init(pre_route);

        let route = helper::route_parented(&mut general_parent, route);

        if target == Target::Directory {
            directory::create(route); 
        continue ; }

        if target == Target::File {
            file::create(route, String::new(), sholl_force);
        continue ; } 
        
        let next_path = routes
                .next()
                .unwrap_or_bayern()
                .msgdln(f!("Could not get the argument for `{route:?}`"))
                .exit(PROCCESSING_EXIT_CODE);

        if target == Target::Symlink {
            symlink::create(route, next_path);
        continue ; } 

        if target == Target::Hardlink {
            hardlink::create(route, next_path);
        continue ; }

        let data = next_path.into_os_string();

        if target == Target::FileAndWrite {
            file::create(route, data, sholl_force);
        continue ; }
    } 
}
