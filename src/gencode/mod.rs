pub mod get_comp_type_manager;
// todo - move this outside of src and make sure this doesn't get added in build

use codegen::{Block, Scope};

use crate::{
    compositions::{banners::banner_basic::BannerCreateReq, CompositionCategory},
    gencode::get_comp_type_manager::setup::{get_composition_type, ArmsBlock, CrudOperation},
};

pub fn impl_composition_type_manager(composition_category: CompositionCategory) {
    let composition_type = get_composition_type(&composition_category);

    let generics = format!("{}Type, {:?}", composition_type, BannerCreateReq {});

    let mut scope = Scope::new();
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
                .arg("composition_type", &composition_type)
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
