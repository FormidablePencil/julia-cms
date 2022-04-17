use codegen::{Block, Scope};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::compositions::{
    banners::{banner_basic::BannerCreateReq, BannerType},
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
    fn get_arm(
        &self,
        enum_name: String,
        enum_type_name: String,
        comp_type_struct_name: &str,
    ) -> String {
        let arm_left = format!("{}::{} =>", enum_name, enum_type_name);
        let arm_right = format!(
            "{}::{}(create_request, layout_id, author_id),",
            comp_type_struct_name,
            self.get_method_name()
        );

        format!("{} {}", arm_left, arm_right)
    }
}

pub trait ArmsBlock {
    fn add_arms(&mut self, crud_operation: CrudOperation) -> &mut Self;
}

impl ArmsBlock for Block {
    fn add_arms(&mut self, crud_operation: CrudOperation, banner_type: BannerType) -> &mut Self {
        for item in BannerType::iter() {
            self.line(crud_operation.get_arm(
                "BannerType".to_string(),
                format!("{:?}", item),
                "banner_basic",
            ));
        }

        self
    }
}

pub fn get_mod(banner_type: CompositionCategory) -> String {
    match banner_type {
        CompositionCategory::Carousel(comp_type) => match comp_type {
            crate::compositions::carousels::CarouselType::Basic => todo!(),
            crate::compositions::carousels::CarouselType::BlurredOverlay => todo!(),
            crate::compositions::carousels::CarouselType::Images => todo!(),
        },
        CompositionCategory::Banner(comp_type) => match comp_type {
            BannerType::Basic => "banner_basic".to_string(),
            BannerType::SomeOtherComp => "some_other_comp".to_string(),
        },
        CompositionCategory::Text(comp_type) => match comp_type {
            crate::compositions::texts::TextType::TextBasic => todo!(),
        },
    }
}
