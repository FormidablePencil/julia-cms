use std::any::Any;

use strum_macros::{EnumIter, EnumString};

use self::{
    carousel_basic::CarouselBasicCreateReq,
    carousel_blurred_overlay::CarouselBlurredOverlayCreateReq,
    carousel_images::CarouselOfImagesCreateReq, carousel_type::CarouselType,
};

use super::{texts::CompositionTypeManager, UpdateDataOfComposition};

pub mod carousel_basic;
pub mod carousel_blurred_overlay;
pub mod carousel_images;
pub mod carousel_type;

pub mod manager;

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
            CarouselType::Basic => carousel_basic::get_private(composition_source_id, author_id),
            CarouselType::BlurredOverlay => todo!(),
            CarouselType::Images => todo!(),
        }
    }

    fn create(
        &self,
        comp_type: CarouselType,
        create_request: Box<dyn Any>,
        layout_id: u128,
        author_id: u128,
    ) {
        match comp_type {
            CarouselType::Basic => match create_request.downcast_ref::<CarouselBasicCreateReq>() {
                Some(req) => carousel_basic::create(req, layout_id, author_id),
                None => panic!("&a isn't a B!"),
            },

            CarouselType::BlurredOverlay => {
                match create_request.downcast_ref::<CarouselBlurredOverlayCreateReq>() {
                    Some(req) => carousel_blurred_overlay::create(req, layout_id, author_id),
                    None => panic!("&a isn't a B!"),
                }
            }

            CarouselType::Images => {
                match create_request.downcast_ref::<CarouselOfImagesCreateReq>() {
                    Some(res) => carousel_images::create(res, layout_id, author_id),
                    None => panic!("&a isn't a B!"),
                }
            }
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
