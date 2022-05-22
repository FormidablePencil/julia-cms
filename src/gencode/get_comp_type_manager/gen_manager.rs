use codegen::{Scope, Block};

use crate::{
    compositions::CompositionCategory,
    gencode::get_comp_type_manager::{helpers::{get_composition_name, get_composition_create_request}, crud_operation::CrudOperation, arms_block::ArmsBlock},
};

pub fn gen_manager(scope: &mut Scope, composition_category: CompositionCategory) {
    let composition_type = get_composition_name(&composition_category, true);
    let composition_name = get_composition_name(&composition_category, false);
    let (_, create_request) = get_composition_create_request(&composition_category);
    let generics = format!("{composition_type}, {create_request}");

    scope
        .new_impl(format!("{}sManager", composition_name).as_str())
        .impl_trait(format!("CompositionTypeManager<{}>", generics))
        .push_fn(
            Scope::new()
                .new_fn("get_public")
              .ret("bool")  
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
}
