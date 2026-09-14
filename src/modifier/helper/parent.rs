use crate::modder::*;

pub fn general_parent<Iter>(routes: &mut Iter, sholl: &Sholl) -> Option<PathBuf>
where
    Iter: Iterator<Item = PathBuf>,
{
    if sholl.parent {
        let parent = routes.next().unwrap_or_bye(|bayern| {
            bayern
                .msgln("No parent was defined")
                .exit(PROCCESSING_EXIT_CODE)
        });
        Some(parent)
    } else {
        None
    }
}

pub fn route_parented(parent: &mut Option<PathBuf>, route: PathBuf) -> PathBuf
{
    if let Some(parent) = parent.as_mut() {
            let mut new_parent = parent.clone();
            new_parent.push(route);
            new_parent
        } else {
            route
        }
}
