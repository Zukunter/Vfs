use crate::modder::*;
use ztd::vfs;
use std::io::Write;

pub fn create<AsPath, AsStr>(file: AsPath, data: AsStr, sholl: &Sholl) 
where 
    AsPath: AsRef<Path>,
    AsStr: AsRef<str>
{
    let file_ref = file.as_ref();
    let data_ref = data.as_ref();

    let mut file_writer = vfs::create_file_all(file_ref, sholl.force)
    .unwrap_or_bye(|bayern, e| {
        bayern
            .msgdln(f!("Could not create file `{file_ref:?}`"))
            .msgd(f!("{e:?}"))
            .exit(TOUCHING_EXIT_CODE)    
    });

    write!(&mut file_writer, "{data_ref}").unwrap_or_bye(|bayern, e| {
        bayern
            .msgdln(f!("Could not write onto file `{file_ref:?}`"))
            .msgd(f!("{e:?}"))
            .exit(TOUCHING_EXIT_CODE)
    });
}
