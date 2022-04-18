pub mod get_comp_type_manager;
pub mod main;
// todo - move this outside of src and make sure this doesn't get added in build

use std::borrow::BorrowMut;

use codegen::{Block, Scope, Variant};

use crate::{
    compositions::{
        banners::banner_basic::{self, BannerCreateReq},
        CompositionCategory,
    },
    gencode::get_comp_type_manager::setup::{
        get_composition_type, get_mod, write_to_file, ArmsBlock, CrudOperation,
    },
};

pub fn impl_composition_type_manager(
    composition_category: CompositionCategory,
    create_request: &str,
    create_request_path: &str,
) {
    let composition_type = get_composition_type(&composition_category);

    let generics = format!("{composition_type}Type, {create_request}");

    let mut scope = Scope::new();
    scope.import("crate::compositions::texts", "CompositionTypeManager");
    scope.import(create_request_path, create_request);
    scope.import("strum_macros", "EnumIter");
    scope.import("strum_macros", "EnumString");
    // scope.import(
    //     format!("self::{}", get_mod(&composition_category)).as_str(),
    //     create_request,
    // );
    scope.import("super", "UpdateDataOfComposition");
    scope.import("super", "carousel_basic");
    scope.import("super", "carousel_blurred_overlay");
    scope.import("super", "carousel_images");
    scope.import(create_request_path, create_request);
    scope.import(
        "super::carousel_type",
        format!("{composition_type}Type").as_str(),
    );

    // scope
    //     .new_enum(format!("{composition_type}Type").as_str())
    //     .vis("pub")
    //     .derive("Debug")
    //     .derive("EnumIter")
    //     .derive("EnumString")
    //     .new_variant("Basic, SomeOtherComp");

    scope
        .new_struct(format!("{composition_type}sManager").as_str())
        .vis("pub");

    scope
        .new_impl(format!("{}sManager", composition_type).as_str())
        .impl_trait(format!("CompositionTypeManager<{}>", generics))
        .push_fn(
            Scope::new()
                .new_fn("get_public")
                .arg_ref_self()
                .arg("composition_type", format!("{composition_type}Type"))
                .arg("composition_source_id", "u128")
                .push_block(
                    Block::new("match composition_type")
                        .add_arms(CrudOperation::GetPublic, &composition_category)
                        .to_owned(),
                )
                .to_owned(),
        )
        .push_fn(
            Scope::new()
                .new_fn("get_private")
                .arg_ref_self()
                .arg("composition_type", format!("{composition_type}Type"))
                .arg("composition_source_id", "u128")
                .arg("author_id", "u128")
                .push_block(
                    Block::new("match composition_type")
                        .add_arms(CrudOperation::GetPrivate, &composition_category)
                        .to_owned(),
                )
                .to_owned(),
        )
        .push_fn(
            Scope::new()
                .new_fn("create")
                .arg_ref_self()
                .arg("composition_type", format!("{composition_type}Type"))
                .arg("create_request", create_request)
                .arg("layout_id", "u128")
                .arg("author_id", "u128")
                .push_block(
                    Block::new("match composition_type")
                        .add_arms(CrudOperation::Create, &composition_category)
                        .to_owned(),
                )
                .to_owned(),
        )
        .push_fn(
            Scope::new()
                .new_fn("update")
                .arg_ref_self()
                .arg("composition_type", format!("{composition_type}Type"))
                .arg("composition_update_que", "Vec<UpdateDataOfComposition>")
                .arg("composition_source_id", "u128")
                .arg("author_id", "u128")
                .push_block(
                    Block::new("match composition_type")
                        .add_arms(CrudOperation::Update, &composition_category)
                        .to_owned(),
                )
                .to_owned(),
        )
        .push_fn(
            Scope::new()
                .new_fn("delete")
                .arg_ref_self()
                .arg("composition_type", format!("{composition_type}Type"))
                .arg("composition_source_id", "u128")
                .arg("author_id", "u128")
                .push_block(
                    Block::new("match composition_type")
                        .add_arms(CrudOperation::Delete, &composition_category)
                        .to_owned(),
                )
                .to_owned(),
        );

    println!("{}", scope.to_string());

    match write_to_file(
        format!("src/compositions/{}/manager.rs", "carousels").as_str(),
        &mut scope.to_string(),
    ) {
        Ok(_) => todo!("success"),
        Err(_) => todo!("failed"),
    }
}

#[cfg(test)]
mod carousel {
    use crate::compositions::{carousels::carousel_type::CarouselType, CompositionCategory};

    use super::impl_composition_type_manager;

    #[test]
    fn add() {
        impl_composition_type_manager(
            CompositionCategory::Carousel(CarouselType::Basic),
            "CarouselBasicCreateReq",
            "super::carousel_basic",
        );
    }

    #[test]
    fn delete() {
        // panic!("Make this test fail");
    }
}
