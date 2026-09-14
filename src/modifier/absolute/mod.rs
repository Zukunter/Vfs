use crate::modder::*;

pub fn absolute<IterPathBuf>(routes: &mut IterPathBuf, sholl: &Sholl) 
where 
    IterPathBuf: Iterator<Item = PathBuf>
{
    let mut general_parent = helper::general_parent(routes, sholl);

    while let Some(route) = routes.next() {
        let route = helper::route_parented(&mut general_parent, route);
        let cano_route = fs::canonicalize(&route).unwrap_or_bye(|bayern, err|{
            let msg_err = err.to_string();            
            bayern
                .msgdln(f!("Could not canonicalize the route `{route:?}`"))
                .msgdln(f!("{msg_err}"))
                .exit(PARSING_EXIT_CODE)
        });

        println!("{route:?} -> {cano_route:?}");
    }
}
