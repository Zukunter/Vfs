use std::{
    path::PathBuf,
    fs,
};

pub fn canonicalized_routes(pre_routes: &mut Vec<String>, sholl_cano: bool) -> Vec<PathBuf> {

    if !sholl_cano { 
        return pre_routes.iter()
            .map(|route| PathBuf::from(route))
            .collect();
    }

    let mut canonicalized_routes: Vec<PathBuf> = Vec::new();

    for route in pre_routes {
        let sections = route.split("-/");

        let mut canonicalized_route = PathBuf::new();

        for section in sections {
            let canonicalized_section = fs::canonicalize(section)
                .unwrap_or(section.into());

            canonicalized_route.push(canonicalized_section);
        }
        canonicalized_routes.push(canonicalized_route);
    }
return canonicalized_routes; }
