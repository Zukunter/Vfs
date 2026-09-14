use crate::modder::*;

mod file; 
mod directory;
mod symlink;
mod hardlink;

pub fn create<IterPathBuf>(routes: &mut IterPathBuf, sholl: &Sholl) 
where 
    IterPathBuf: Iterator<Item = PathBuf>
{

    let mut general_parent = helper::general_parent(routes, sholl);

    while let Some(pre_route) = routes.next() {
        let (route, target) = Target::init(pre_route);

        let route = helper::route_parented(&mut general_parent, route);

        if target == Target::Directory {
            directory::create(route);
        } 
        else if target == Target::File {
            let data = if sholl.write { 
                routes.next()
                .unwrap_or_bayern()
                .msgd("No data was passed")
                .exit(PROCCESSING_EXIT_CODE)
                .try_to_string()
                .unwrap_or_bayern()
                .msgd("Could not parser `&PathBuf` onto String")
                .exit(PARSING_EXIT_CODE)
            } else { String::new() };
            file::create(route, data, sholl);
        } else {
            let next_path = routes.next().unwrap_or_bayern()
                .msgd("No path to point to was found")
                .exit(PROCCESSING_EXIT_CODE);
            match target {
                Target::Symlink => symlink::create(route, next_path),
                Target::Hardlink => hardlink::create(route, next_path),
                _ => unreachable!()
            }
        }
    } 
}
