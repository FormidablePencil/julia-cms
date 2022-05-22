use std::{fs::File, io::Write};

mod arms_block;
mod crud_operation;
mod gen_manager;
mod helpers;
mod import_mods;
mod manager_dependents;

use codegen::{Block, Scope};

use crate::gencode::get_comp_type_manager::helpers::get_composition_create_request;
use crate::gencode::get_comp_type_manager::import_mods::import_composition_mods;
use crate::{
    compositions::CompositionCategory,
    gencode::get_comp_type_manager::{
        arms_block::ArmsBlock,
        crud_operation::CrudOperation,
        helpers::{get_composition_name, import_composition_models},
    },
    get_composition_name,
};

use self::gen_manager::gen_manager;
use self::manager_dependents::manager_dependents;

// todo - move this outside of src and make sure this doesn't get added in build

pub fn write_to_file(file_name: &str, contents: &mut String) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(contents.as_bytes())?;
    // assert_eq!(contents, "Hello, world!");
    Ok(())
}

pub fn impl_composition_type_manager(composition_category: CompositionCategory) {
    let composition_name = get_composition_name!(&composition_category).to_ascii_lowercase();
    let model_name = format!("{composition_name}s");

    let mut scope = Scope::new();

    manager_dependents(&mut scope, composition_category);

    gen_manager(&mut scope, composition_category);

    match write_to_file(
        format!("src/compositions/{}/manager.rs", model_name).as_str(),
        &mut scope.to_string(),
    ) {
        Ok(_) => print!("success"),
        Err(_) => todo!("failed"),
    }
}

#[cfg(test)]
mod carousel {
    use crate::compositions::banners::banner_type::BannerType;
    use crate::compositions::CompositionCategory;

    use super::impl_composition_type_manager;

    #[test]
    fn construct_manager() {
        impl_composition_type_manager(
            CompositionCategory::Banner(BannerType::Basic),
            // CompositionCategory::Carousel(CarouselType::Basic),
        );
    }
}
