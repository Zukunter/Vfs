use crate::modder::*;
use std::fs;

pub fn cut<IterPathBuf>(routes: &mut IterPathBuf, sholl: &Sholl) 
where 
    IterPathBuf: Iterator<Item = PathBuf>
{
    let mut general_parent = helper::general_parent(routes, sholl);

    while let Some(pre_route) = routes.next() {
        let route = helper::route_parented(&mut general_parent, pre_route);
        let new_name = routes.next().unwrap_or_bayern()
            .msgdln("No new name was sent")
            .exit(TOUCHING_EXIT_CODE);

        fs::rename(&route, &new_name).unwrap_or_bye(|bayern, err|{
            let msg_err = err.to_string();
            bayern
                .msgdln(f!("Could not rename the entry `{route:?}` as `{new_name:?}`"))
                .msgdln(f!("{msg_err}"))
                .exit(TOUCHING_EXIT_CODE)
        });
    }
}
