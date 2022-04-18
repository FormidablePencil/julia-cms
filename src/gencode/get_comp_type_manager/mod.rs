use std::{fs::File, io::Write};

pub mod crud_operation;
pub mod arms_block;
pub mod helpers;
pub mod util;

pub fn write_to_file(file_name: &str, contents: &mut String) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(contents.as_bytes())?;
    // assert_eq!(contents, "Hello, world!");
    Ok(())
}