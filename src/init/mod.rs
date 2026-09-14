use crate::modifier;
use crate::modder::*;
use std::env;

mod cmd; use cmd::CmdLower;

pub fn init() {
    let mut args = env::args().skip(1);
    let cmd_lower = CmdLower::init(&mut args);
    let mut routes: Vec<String> = args.collect();


    let action = Action::init(cmd_lower.action);
    let sholl = Sholl::init(cmd_lower.sholl);

    let mut canonicalized_routes_iter = helper::canonicalized_routes(
        &mut routes, 
        &sholl
    ).into_iter();

    match action {
        Create => modifier::create(&mut canonicalized_routes_iter, &sholl),
        Remove => modifier::remove(&mut canonicalized_routes_iter, &sholl),
        Cut => modifier::cut(&mut canonicalized_routes_iter, &sholl),
        Absolute => modifier::absolute(&mut canonicalized_routes_iter, &sholl),
        _ => {}
    }
}


