use std::{
    fs::File,
    io::{BufReader, Read, Write},
    ops::DerefMut,
};

use codegen::Block;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    compositions::{
        banners::BannerType, carousels::carousel_type::CarouselType, texts::TextType,
        CompositionCategory,
    },
    get_composition_name,
};

use super::util::get_composition_name;

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

    fn get_arm_for_create() {}
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

    fn get_composition_create_request(composition_category: &CompositionCategory) -> String;
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
        let create_request = Self::get_composition_create_request(&composition_category);

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

    fn get_composition_create_request(composition_category: &CompositionCategory) -> String {
        match composition_category {
            CompositionCategory::Carousel(comp_type) => match comp_type {
                CarouselType::Basic => "CarouselBasicCreateReq",
                CarouselType::BlurredOverlay => "CarouselBlurredOverlayCreateReq",
                CarouselType::Images => "CarouselOfImagesCreateReq",
            },
            CompositionCategory::Banner(_) => todo!(),
            CompositionCategory::Text(_) => todo!(),
        }
        .to_string()
    }
}

pub fn get_mod(composition_category: &CompositionCategory) -> String {
    match composition_category {
        CompositionCategory::Carousel(comp_type) => match comp_type {
            CarouselType::Basic => "carousel_basic".to_string(),
            CarouselType::BlurredOverlay => "carousel_blurred_overlay".to_string(),
            CarouselType::Images => "carousel_images".to_string(),
        },
        CompositionCategory::Banner(comp_type) => match comp_type {
            BannerType::Basic => "banner_basic".to_string(),
            BannerType::SomeOtherComp => "some_other_comp".to_string(),
        },
        CompositionCategory::Text(comp_type) => match comp_type {
            crate::compositions::texts::TextType::Basic => "text_basic".to_string(),
        },
    }
}

// // todo - same as above with "Type" added
// fn get_composition_type(composition_category: &CompositionCategory) -> String {
//     match composition_category {
//         CompositionCategory::Carousel(_) => "CarouselType",
//         CompositionCategory::Banner(_) => "BannerType",
//         CompositionCategory::Text(_) => "TextType",
//     }
//     .to_string()
// }

pub fn write_to_file(file_name: &str, contents: &mut String) -> std::io::Result<()> {
    let mut file = File::create(file_name)?;
    file.write_all(contents.as_bytes())?;
    // assert_eq!(contents, "Hello, world!");
    Ok(())
}
