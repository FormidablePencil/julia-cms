use codegen::{Block, Scope};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::compositions::{
    banners::{banner_basic::BannerCreateReq, BannerType},
    carousels::CarouselType,
    texts::TextType,
    CompositionCategory,
};

#[derive(Debug, EnumIter)]
pub enum CrudOperation {
    GetPublic,
    GetPrivate,
    Create,
    Update,
    Delete,
}

impl CrudOperation {
    fn get_method_name(&self) -> String {
        match self {
            CrudOperation::GetPublic => "get_public".to_string(),
            CrudOperation::GetPrivate => "get_private".to_string(),
            CrudOperation::Create => "create".to_string(),
            CrudOperation::Update => "update".to_string(),
            CrudOperation::Delete => "delete".to_string(),
        }
    }

    fn get_args_for_method(&self) -> String {
        match self {
            CrudOperation::GetPublic => "(composition_source_id)".to_string(),
            CrudOperation::GetPrivate => "(composition_source_id, author_id)".to_string(),
            CrudOperation::Create => "(create_request, layout_id, author_id)".to_string(),
            CrudOperation::Update => {
                "(composition_update_que, composition_source_id, author_id)".to_string()
            }
            CrudOperation::Delete => "(composition_source_id, author_id)".to_string(),
        }
    }

    fn get_arm(
        &self,
        enum_name: String,
        enum_type_name: String,
        comp_type_struct_name: String,
    ) -> String {
        let arm_left = format!("{}::{} =>", enum_name, enum_type_name);
        let arm_right = format!(
            "{}::{}{},",
            comp_type_struct_name,
            self.get_method_name(),
            self.get_args_for_method()
        );

        format!("{} {}", arm_left, arm_right)
    }
}

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
        let comp_type_name = match composition_category {
            CompositionCategory::Carousel(_) => "CarouselType",
            CompositionCategory::Banner(_) => "BannerType",
            CompositionCategory::Text(_) => "TextType",
        };
        // "CarouselType".to_string(),
        self.line(crud_operation.get_arm(
            comp_type_name.to_string(),
            enum_type_name,
            get_mod(&composition_category),
        ));

        self
    }
}

pub fn get_mod(composition_category: &CompositionCategory) -> String {
    match composition_category {
        CompositionCategory::Carousel(comp_type) => match comp_type {
            crate::compositions::carousels::CarouselType::Basic => "carousel_basic".to_string(),
            crate::compositions::carousels::CarouselType::BlurredOverlay => {
                "carousel_blurred_overlay".to_string()
            }
            crate::compositions::carousels::CarouselType::Images => "carousel_image".to_string(),
        },
        CompositionCategory::Banner(comp_type) => match comp_type {
            BannerType::Basic => "banner_basic".to_string(),
            BannerType::SomeOtherComp => "some_other_comp".to_string(),
        },
        CompositionCategory::Text(comp_type) => match comp_type {
            crate::compositions::texts::TextType::TextBasic => "text_basic".to_string(),
        },
    }
}

pub fn get_composition_type(composition_category: &CompositionCategory) -> String {
    match composition_category {
        CompositionCategory::Carousel(_) => "Carousel",
        CompositionCategory::Banner(_) => "Banner",
        CompositionCategory::Text(_) => "Text",
    }
    .to_string()
}
