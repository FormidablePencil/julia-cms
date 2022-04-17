pub mod banners;
pub mod carousels;
pub mod texts;

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

pub enum CompositionCategory {
    Carousel(CarouselType),
    Banner(BannerType),
    Text(TextType),
}

use self::{
    banners::{BannerManager, BannerType},
    carousels::{CarouselManager, CarouselType},
    texts::{text_basic::TextBasicCreateReq, CompositionTypeManager, TextManager, TextType},
};

impl CategoryManager {
    fn get_public(
        &self,
        comp_category: CompositionCategory,
        comp_type: u128,
        composition_source_id: u128,
    ) {
        match comp_category {
            CompositionCategory::Carousel(comp_type) => self
                .carousel_manager
                .get_public(comp_type, composition_source_id),

            CompositionCategory::Banner(comp_type) => todo!(),/* self
                .banner_manager
                .get_public(comp_type, composition_source_id), */

            CompositionCategory::Text(comp_type) => self
                .text_manager
                .get_public(comp_type, composition_source_id),
        }
    }

    fn get_private(
        &self,
        comp_category: CompositionCategory,
        composition_source_id: u128,
        author_id: u128,
    ) {
        match comp_category {
            CompositionCategory::Text(comp_type) => {
                self.text_manager
                    .get_private(comp_type, composition_source_id, author_id)
            }
            CompositionCategory::Carousel(_) => todo!(),
            CompositionCategory::Banner(_) => todo!(),
        }
    }

    fn create(
        &self,
        comp_category: CompositionCategory,
        create_request: TextBasicCreateReq,
        layout_id: u128,
        author_id: u128,
    ) {
    }

    fn update(
        &self,
        comp_category: CompositionCategory,
        composition_update_que: Vec<UpdateDataOfComposition>,
        layout_id: u128,
        author_id: u128,
    ) {
    }

    fn delete(
        &self,
        comp_category: CompositionCategory,
        composition_source_id: u128,
        author_id: u128,
    ) {
    }
}