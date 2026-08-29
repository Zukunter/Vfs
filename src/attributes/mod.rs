mod action; use action::get_action;
mod target; use target::get_target;
mod sholl; use sholl::get_sholl;

use ztd::Bayern;

pub use sholl::Sholl;

pub fn get_attributes(bayern: &mut Bayern, cmd: &String) -> (u8, u8, Sholl) {
    (get_action(bayern, cmd), get_target(bayern, cmd), get_sholl(cmd) )
}
