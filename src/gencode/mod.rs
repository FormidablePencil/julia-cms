pub mod get_comp_type_manager;
// todo - move this outside of src and make sure this doesn't get added in build

use codegen::{Block, Scope};

use crate::{
    compositions::banners::banner_basic::BannerCreateReq,
    gencode::get_comp_type_manager::setup::{ArmsBlock, CrudOperation},
};

pub fn impl_composition_type_manager() {
    let composition_type = "BannerType";
    let generics = format!("BannerType, {:?}", BannerCreateReq {});

    let mut scope = Scope::new();
    scope
        .new_impl("BannerManager")
        .impl_trait(format!("CompositionTypeManager<{}>", generics))
        .push_fn(
            Scope::new()
                .new_fn("get_public")
                .arg_ref_self()
                .arg("composition_type", composition_type)
                .arg("composition_source_id", "u128")
                .push_block(
                    Block::new("match composition_type")
                        .add_arms(CrudOperation::GetPublic)
                        .to_owned(),
                )
                .to_owned(),
        )
        .push_fn(
            Scope::new()
                .new_fn("get_private")
                .arg_ref_self()
                .arg("composition_type", composition_type)
                .arg("composition_source_id", "u128")
                .arg("author_id", "u128")
                .push_block(
                    Block::new("match composition_type")
                        .add_arms(CrudOperation::GetPrivate)
                        .to_owned(),
                )
                .to_owned(),
        )
        .push_fn(
            Scope::new()
                .new_fn("create")
                .arg_ref_self()
                .arg("composition_type", composition_type)
                .arg("author_id", "u128")
                .to_owned(),
        )
        .push_fn(
            Scope::new()
                .new_fn("update")
                .arg_ref_self()
                .arg("composition_type", composition_type)
                .arg("composition_source_id", "u128")
                .arg("author_id", "u128")
                .to_owned(),
        )
        .push_fn(
            Scope::new()
                .new_fn("delete")
                .arg_ref_self()
                .arg("composition_type", composition_type)
                .arg("composition_source_id", "u128")
                .arg("author_id", "u128")
                .to_owned(),
        );
    println!("{}", scope.to_string());
}
