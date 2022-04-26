use codegen::{Block, Scope};

use crate::gencode::get_comp_type_manager::helpers::get_composition_create_request;
use crate::gencode::get_comp_type_manager::import_mods::{self, import_composition_mods};
use crate::{
    compositions::CompositionCategory,
    gencode::get_comp_type_manager::{
        arms_block::ArmsBlock,
        crud_operation::CrudOperation,
        helpers::{get_composition_name, import_composition_models},
        write_to_file,
    },
    get_composition_name,
};

pub mod get_comp_type_manager;
// todo - move this outside of src and make sure this doesn't get added in build

pub fn impl_composition_type_manager(composition_category: CompositionCategory) {
    let composition_type = get_composition_name(&composition_category, true);

    let (_, create_request) = get_composition_create_request(&composition_category);
    let generics = format!("{composition_type}, {create_request}");

    let mut scope = Scope::new();

    scope.import("std::any", "Any");
    scope.import("crate::compositions::texts", "CompositionTypeManager");

    scope.import("strum_macros", "EnumIter");
    scope.import("strum_macros", "EnumString");
    // scope.import(
    //     format!("self::{}", get_mod(&composition_category)).as_str(),
    //     create_request,
    // );
    scope.import("super", "UpdateDataOfComposition");
    let comp_name = get_composition_name!(&composition_category).to_lowercase();
    import_composition_models(&mut scope, &composition_category);
    import_composition_mods(&mut scope, &composition_category);
    // scope.import("super", format!("{comp_name}_basic").as_str());
    // scope.import("super", "carousel_blurred_overlay");
    // scope.import("super", "carousel_images");

    // get_composition_create_request
    import_composition_models(&mut scope, &composition_category);
    scope.import(
        format!("super::{comp_name}_type").as_str(),
        composition_type.as_str(),
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
                .arg("composition_type", &composition_type)
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
                .arg("composition_type", &composition_type)
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
                .arg("composition_type", &composition_type)
                .arg("create_request", "Box<dyn Any>")
                .arg("layout_id", "u128")
                .arg("author_id", "u128")
                .push_block(Block::new("match composition_type")
                    .add_arms_for_create(&CrudOperation::Update, &composition_type, &composition_category).to_owned()
                ).to_owned()
            // .push_block(
            //     Block::new("CarouselType::Basic => match create_request.downcast_ref::<CarouselBasicCreateReq>()")
            //         .line("Some(req) => carousel_basic::create(req, layout_id, author_id),")
            //         .line("None => panic!()")
            //         .to_owned()
            //         // CarouselType::Basic => match create_request.downcast_ref::<CarouselBasicCreateReq>() {
            //         //     Some(req) => carousel_basic::create(req, layout_id, author_id),
            //         //     None => panic!("&a isn't a B!"),
            //         // },
            //         // .add_arms(CrudOperation::Create, &composition_category)
            // ).to_owned())
            // .to_owned(),
        )
        .push_fn(
            Scope::new()
                .new_fn("update")
                .arg_ref_self()
                .arg("composition_type", &composition_type)
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
                .arg("composition_type", &composition_type)
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

    let composition_name = get_composition_name!(&composition_category).to_ascii_lowercase();
    let model_name = format!("{composition_name}s");

    match write_to_file(
        format!("src/compositions/{}/manager.rs", model_name).as_str(),
        &mut scope.to_string(),
    ) {
        Ok(_) => todo!("success"),
        Err(_) => todo!("failed"),
    }
}

#[cfg(test)]
mod carousel {
    use crate::compositions::banners::banner_type::BannerType;
    use crate::compositions::{carousels::carousel_type::CarouselType, CompositionCategory};

    use super::impl_composition_type_manager;

    #[test]
    fn construct_manager() {
        impl_composition_type_manager(
            CompositionCategory::Banner(BannerType::Basic),
            // CompositionCategory::Carousel(CarouselType::Basic),
        );
    }
}
