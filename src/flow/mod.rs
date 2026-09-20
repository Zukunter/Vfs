use std::{
    env
};
use crate::modder::{
    Action::{self, *},
    Sholl,
    helper,
    modifier
};

mod cmd; use cmd::CmdLower;

pub fn init() {
    let mut args = env::args().skip(1);
    let cmd_lower = CmdLower::init(&mut args);
    let mut routes: Vec<String> = args.collect();

    let action = Action::init(cmd_lower.action);
    let sholl = Sholl::init(cmd_lower.sholl);

    let mut canonicalized_routes_iter = helper::canonicalized_routes(
        &mut routes, 
        sholl.cano
    ).into_iter();

    let general_parent = helper::general_parent(
        &mut canonicalized_routes_iter, 
        sholl.parent
    );

    match action {
        Create => modifier::create(general_parent, &mut canonicalized_routes_iter, sholl.force),
        Remove => modifier::remove(general_parent, &mut canonicalized_routes_iter, sholl.force),
        Cut => modifier::cut(general_parent, &mut canonicalized_routes_iter),
        Absolute => modifier::absolute(general_parent, &mut canonicalized_routes_iter),
        Truncate => modifier::truncate(general_parent, &mut canonicalized_routes_iter),
        _ => {}
    }
}
