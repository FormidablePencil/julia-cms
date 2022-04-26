use std::{fs::File, io::Write};

pub mod arms_block;
pub mod crud_operation;
pub mod helpers;
pub mod import_mods;

pub fn write_to_file(file_name: &str, contents: &mut String) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(contents.as_bytes())?;
    // assert_eq!(contents, "Hello, world!");
    Ok(())
}
