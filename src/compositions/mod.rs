extern crate proc_macro;

use std::any::Any;
use carousels::carousel_enums::CarouselResponse;
use texts::text_enums::{TextResponse, TextType};

use crate::compositions::banners::banner_enums::BannerResponse;

use self::{
    banners::{
        banner_enums::BannerType,
        manager::BannerManager,
    },
    carousels::{carousel_enums::CarouselType, CarouselManager},
    manager_impl::CompositionTypeManager,
    texts::{
        manager::TextManager,
        text_basic::TextBasicCreateReq,
    },
};

pub mod banners;
pub mod carousels;
pub mod manager_impl;
pub mod texts;
mod base_comp_result;

// region enum and structs
pub struct UpdateDataOfComposition {
    update_data_of: u128,
    record_update: RecordUpdate,
}

pub struct RecordUpdate {
    record_id: u128,
    update_to: Vec<UpdateColumn>,
}

pub struct UpdateColumn {
    column: u128,
    value: String,
}

struct CategoryManager {
    text_manager: TextManager,
    carousel_manager: CarouselManager,
    banner_manager: BannerManager,
}

#[derive(Debug)]
pub enum CompositionCategory {
    Carousel(CarouselType),
    Banner(BannerType),
    Text(TextType),
}

enum CategoryResponse {
    Carousel(CarouselResponse),
    Banner(BannerResponse),
    Text(TextResponse),
}

// endregion enum and structs

impl CategoryManager {
    fn get_public(
        &self,
        comp_category: CompositionCategory,
        composition_source_id: u128,
    ) -> CategoryResponse {
        match comp_category {
            CompositionCategory::Carousel(comp_type) => CategoryResponse::Carousel(
                self.carousel_manager
                    .get_public(comp_type, composition_source_id)
            ),
            // CompositionCategory::Banner(_) => todo!(),
            CompositionCategory::Banner(comp_type) => CategoryResponse::Banner(
                self.banner_manager
                    .get_public(comp_type, composition_source_id),
            ),
            CompositionCategory::Text(comp_type) => CategoryResponse::Text(
                self.text_manager
                    .get_public(comp_type, composition_source_id),
            ),
        }
    }

    fn get_private(
        &self,
        comp_category: CompositionCategory,
        composition_source_id: u128,
        author_id: u128,
    ) -> CategoryResponse {
        match comp_category {
            CompositionCategory::Carousel(comp_type) => CategoryResponse::Carousel(
                self.carousel_manager
                    .get_private(comp_type, composition_source_id, author_id),
            ),
            // CompositionCategory::Banner(_) => todo!(),
            CompositionCategory::Banner(comp_type) => CategoryResponse::Banner(
                self.banner_manager
                    .get_private(comp_type, composition_source_id, author_id),
            ),
            CompositionCategory::Text(comp_type) => CategoryResponse::Text(
                self.text_manager
                    .get_private(comp_type, composition_source_id, author_id),
            ),
        }
    }

    fn create(
        &self,
        comp_category: CompositionCategory,
        // create_request: TextBasicCreateReq,
        create_request: Box<dyn Any>,
        layout_id: u128,
        author_id: u128,
    ) -> CategoryResponse {
        match comp_category {
            CompositionCategory::Carousel(comp_type) => CategoryResponse::Carousel(
                self.carousel_manager
                    .create(comp_type,create_request, layout_id, author_id),
            ),
            // CompositionCategory::Banner(_) => todo!(),
            CompositionCategory::Banner(comp_type) => CategoryResponse::Banner(
                self.banner_manager
                    .create(comp_type, create_request, layout_id, author_id),
            ),
            CompositionCategory::Text(comp_type) => CategoryResponse::Text(
                self.text_manager
                    .create(comp_type, create_request, layout_id, author_id),
            ),
        }
    }

    // fn update(
    //     &self,
    //     comp_category: CompositionCategory,
    //     composition_update_que: Vec<UpdateDataOfComposition>,
    //     layout_id: u128,
    //     author_id: u128,
    // ) {}

    fn delete(
        &self,
        comp_category: CompositionCategory,
        composition_source_id: u128,
        author_id: u128,
    ) -> bool {
        todo!()
    }
}

// enum OptionCompositionTypes {
//     Carousel(carousels::carousel_type::CarouselTypeIter),
//     Banner(banners::BannerTypeIter),
//     Text(texts::TextTypeIter),
//     None,
// }

// impl CompositionCategory {
//     pub fn iterator() -> Iter<'static, CompositionCategory> {
//         static COMPOSITION_CATEGORY: [CompositionCategory; 3] = [
//             CompositionCategory::Carousel(CarouselType::Basic),
//             CompositionCategory::Banner(BannerType::Basic),
//             CompositionCategory::Text(TextType::Basic),
//         ];
//         COMPOSITION_CATEGORY.iter()
//     }
// }
