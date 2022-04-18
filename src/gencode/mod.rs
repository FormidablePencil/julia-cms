pub mod get_comp_type_manager;
pub mod main;
// todo - move this outside of src and make sure this doesn't get added in build

use std::borrow::BorrowMut;

use codegen::{Block, Scope, Variant};

use crate::{
    compositions::{
        banners::banner_basic::{self, BannerCreateReq},
        carousels::carousel_basic::CarouselBasicCreateReq,
        CompositionCategory,
    },
    gencode::get_comp_type_manager::{
        crud_operation::CrudOperation,
        helpers::{get_mod, import_composition_models, get_composition_name},
        arms_block::ArmsBlock, write_to_file,
    },
    get_composition_name,
};

pub fn impl_composition_type_manager(
    composition_category: CompositionCategory,
    create_request: &str,
    create_request_path: &str,
) {
    let composition_type = get_composition_name(&composition_category, true);

    let generics = format!("{composition_type}, {create_request}");

    let mut scope = Scope::new();

    scope.import("std::any", "Any");
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

    // get_composition_create_request
    scope.import("super", "carousel_images");
    import_composition_models(&mut scope, &composition_category);
    scope.import(create_request_path, create_request);
    scope.import("super::carousel_type", composition_type.as_str());

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
                    .add_arms_for_create(&CrudOperation::Update,&composition_type, &composition_category).to_owned()
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
    fn construct_manager() {
        impl_composition_type_manager(
            CompositionCategory::Carousel(CarouselType::Basic),
            "CarouselBasicCreateReq",
            "super::carousel_basic",
        );
    }
}

use std::any::Any;

pub trait A {
    fn as_any(&self) -> &dyn Any;
}

pub struct B;

impl A for B {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait CarouselRelated {
    fn as_any(&self) -> &dyn Any;
}

impl CarouselRelated for CarouselBasicCreateReq {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn main() {
    let a: Box<dyn A> = Box::new(B);
    // The indirection through `as_any` is because using `downcast_ref`
    // on `Box<A>` *directly* only lets us downcast back to `&A` again.
    // The method ensures we get an `Any` vtable that lets us downcast
    // back to the original, concrete type.
    let b: &B = match a.as_any().downcast_ref::<B>() {
        Some(b) => b,
        None => panic!("&a isn't a B!"),
    };
}
