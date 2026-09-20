use ztd::{
    kern::*,
    vfs
};
use std::{
    path::Path,
    ffi::OsStr, 
    io::Write
};
use crate::modder::{
    TOUCHING_EXIT_CODE
};

pub fn create<AsPath, AsOsStr>(file: AsPath, data: AsOsStr, sholl_force: bool) 
where 
    AsPath: AsRef<Path>,
    AsOsStr: AsRef<OsStr>
{
    let file_ref = file.as_ref();
    let data_ref = data.as_ref();

    let mut file_writer = vfs::create_file_all(file_ref, sholl_force)
    .unwrap_or_bye(|bayern, err| {
        let msg_err = err.to_string();
        bayern
        .msgdln(f!("Could not create file `{file_ref:?}`"))
        .msgd(f!("{msg_err}"))
        .exit(TOUCHING_EXIT_CODE)    
    });

    write!(&mut file_writer, "{data_ref:?}")
    .unwrap_or_bye(|bayern, err| {
        let msg_err = err.to_string();
        bayern
        .msgdln(f!("Could not write onto file `{file_ref:?}`"))
        .msgd(f!("{msg_err}"))
        .exit(TOUCHING_EXIT_CODE)
    });
}
