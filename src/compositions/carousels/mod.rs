use strum_macros::{EnumString, EnumIter};

use self::carousel_basic::CarouselBasicCreateReq;

use super::{texts::CompositionTypeManager, UpdateDataOfComposition};

pub mod carousel_basic;
pub mod carousel_blurred_overlay;
pub mod carousel_of_images;

#[derive(Debug, EnumIter, EnumString)]
pub enum CarouselType {
    Basic,
    BlurredOverlay,
    Images,
}

pub struct CarouselManager {}

impl CompositionTypeManager<CarouselType, CarouselBasicCreateReq> for CarouselManager {
    // use crate::compositions::UpdateDataOfComposition;

    // use super::carousel_basic::{self, CarouselBasicCreateReq};

    fn get_public(&self, comp_type: CarouselType, composition_source_id: u128) {
        match comp_type {
            CarouselType::Basic => carousel_basic::get_public(composition_source_id),
            CarouselType::BlurredOverlay => todo!(),
            CarouselType::Images => todo!(),
        }
    }

    fn get_private(&self, comp_type: CarouselType, composition_source_id: u128, author_id: u128) {
        match comp_type {
            CarouselType::Basic => {
                carousel_basic::get_private(composition_source_id, author_id)
            }
            CarouselType::BlurredOverlay => todo!(),
            CarouselType::Images => todo!(),
        }
    }

    fn create(
        &self,
        comp_type: CarouselType,
        create_request: CarouselBasicCreateReq,
        layout_id: u128,
        author_id: u128,
    ) {
        match comp_type {
            CarouselType::Basic => {
                carousel_basic::create(create_request, layout_id, author_id)
            }
            CarouselType::BlurredOverlay => todo!(),
            CarouselType::Images => todo!(),
        }
    }

    fn update(
        &self,
        comp_type: CarouselType,
        composition_update_que: Vec<UpdateDataOfComposition>,
        layout_id: u128,
        author_id: u128,
    ) {
        match comp_type {
            CarouselType::Basic => {
                carousel_basic::update(composition_update_que, layout_id, author_id)
            }
            CarouselType::BlurredOverlay => todo!(),
            CarouselType::Images => todo!(),
        }
    }

    fn delete(&self, comp_type: CarouselType, composition_source_id: u128, author_id: u128) {
        match comp_type {
            CarouselType::Basic => carousel_basic::delete(composition_source_id, author_id),
            CarouselType::BlurredOverlay => todo!(),
            CarouselType::Images => todo!(),
        }
    }
}
