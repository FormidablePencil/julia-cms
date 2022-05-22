use codegen::Block;
use strum::IntoEnumIterator;

use crate::compositions::banners::banner_type::BannerType;
use crate::compositions::texts::manager::TextType;
use crate::gencode::gen_managers::helpers::get_composition_create_request;
use crate::{
    compositions::{carousels::carousel_type::CarouselType, CompositionCategory},
    get_composition_name,
};

use super::{
    crud_operation::CrudOperation,
    helpers::{get_composition_name, get_mod},
};

pub trait ArmsBlock {
    fn add_arms(
        &mut self,
        crud_operation: CrudOperation,
        composition_category: &CompositionCategory,
    ) -> &mut Self;

    fn add_arm(
        &mut self,
        crud_operation: &CrudOperation,
        enum_type_name: String,
        composition_category: CompositionCategory,
    ) -> &mut Self;

    fn add_arm_for_create(
        &mut self,
        crud_operation: &CrudOperation,
        enum_type_name: String,
        composition_category: CompositionCategory,
    ) -> &mut Self;

    fn add_arms_for_create(
        &mut self,
        crud_operation: &CrudOperation,
        enum_type_name: &String,
        composition_category: &CompositionCategory,
    ) -> &mut Self;

    // fn get_composition_create_request(composition_category: &CompositionCategory) -> String;
}

impl ArmsBlock for Block {
    fn add_arms(
        &mut self,
        crud_operation: CrudOperation,
        composition_category: &CompositionCategory,
    ) -> &mut Self {
        match composition_category {
            CompositionCategory::Carousel(_) => {
                for item in CarouselType::iter() {
                    self.add_arm(
                        &crud_operation,
                        format!("{:?}", item),
                        CompositionCategory::Carousel(item),
                    );
                }
            }
            CompositionCategory::Banner(_) => {
                for item in BannerType::iter() {
                    self.add_arm(
                        &crud_operation,
                        format!("{:?}", item),
                        CompositionCategory::Banner(item),
                    );
                }
            }
            CompositionCategory::Text(_) => {
                for item in TextType::iter() {
                    self.add_arm(
                        &crud_operation,
                        format!("{:?}", item),
                        CompositionCategory::Text(item),
                    );
                }
            }
        }

        self
    }

    fn add_arm(
        &mut self,
        crud_operation: &CrudOperation,
        enum_type_name: String,
        composition_category: CompositionCategory,
    ) -> &mut Self {
        let comp_type_name = get_composition_name!(&composition_category);
        self.line(crud_operation.get_arm(
            get_composition_name(&composition_category, true),
            enum_type_name,
            get_mod(&composition_category),
        ));

        self
    }

    fn add_arm_for_create(
        &mut self,
        crud_operation: &CrudOperation,
        enum_type_name: String,
        composition_category: CompositionCategory,
    ) -> &mut Self {
        let comp_type_name = get_composition_name(&composition_category, true);
        let (_, create_request) = get_composition_create_request::get_composition_create_request(&composition_category);

        let matcher = format!("{comp_type_name}::{enum_type_name} => match create_request.downcast_ref::<{create_request}>()");

        self.push_block(
            Block::new(matcher.as_str())
                .line(format!(
                    "Some(req) => {}::create(req, layout_id, author_id),",
                    get_mod(&composition_category)
                ))
                .line("None => panic!(\"failed...\")")
                .to_owned(),
        )
    }

    fn add_arms_for_create(
        &mut self,
        crud_operation: &CrudOperation,
        enum_type_name: &String,
        composition_category: &CompositionCategory,
    ) -> &mut Self {
        match composition_category {
            CompositionCategory::Carousel(_) => {
                for item in CarouselType::iter() {
                    self.add_arm_for_create(
                        &crud_operation,
                        format!("{:?}", item),
                        CompositionCategory::Carousel(item),
                    );
                }
            }
            CompositionCategory::Banner(_) => {
                for item in BannerType::iter() {
                    self.add_arm_for_create(
                        &crud_operation,
                        format!("{:?}", item),
                        CompositionCategory::Banner(item),
                    );
                }
            }
            CompositionCategory::Text(_) => {
                for item in TextType::iter() {
                    self.add_arm_for_create(
                        &crud_operation,
                        format!("{:?}", item),
                        CompositionCategory::Text(item),
                    );
                }
            }
        }

        self
    }
}
