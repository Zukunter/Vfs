use crate::modder::*;

mod directory;
mod file;
pub fn remove<IterPathBuf>(routes: &mut IterPathBuf, sholl: &Sholl) 
where 
    IterPathBuf: Iterator<Item = PathBuf> 
{
    let mut general_parent = helper::general_parent(routes, sholl);
    while let Some(pre_route) =  routes.next() {
        let (route, target) = Target::init(pre_route);
        let route = helper::route_parented(&mut general_parent, route);

        if target == Target::Directory {
            directory::remove(route, sholl);
        } else {
            file::remove(route)
        }
    }
}
