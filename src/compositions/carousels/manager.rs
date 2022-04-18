use super::carousel_basic::CarouselBasicCreateReq;
use super::carousel_blurred_overlay::CarouselBlurredOverlayCreateReq;
use super::carousel_images::CarouselOfImagesCreateReq;
use super::{carousel_basic, carousel_blurred_overlay, carousel_images, UpdateDataOfComposition};
use crate::compositions::texts::CompositionTypeManager;
use std::any::Any;
use strum_macros::{EnumIter, EnumString};
// use Carousel(Basic)::Carousel(Basic);
// use Carousel(BlurredOverlay)::Carousel(BlurredOverlay);
// use Carousel(Images)::Carousel(Images);
use super::carousel_type::CarouselType;

pub struct CarouselTypesManager;

impl CompositionTypeManager<CarouselType, CarouselBasicCreateReq> for CarouselTypesManager {
    fn get_public(&self, composition_type: CarouselType, composition_source_id: u128) {
        match composition_type {
            CarouselType::Basic => carousel_basic::get_public(composition_source_id),
            CarouselType::BlurredOverlay => {
                carousel_blurred_overlay::get_public(composition_source_id)
            }
            CarouselType::Images => carousel_images::get_public(composition_source_id),
        }
    }

    fn get_private(
        &self,
        composition_type: CarouselType,
        composition_source_id: u128,
        author_id: u128,
    ) {
        match composition_type {
            CarouselType::Basic => carousel_basic::get_private(composition_source_id, author_id),
            CarouselType::BlurredOverlay => {
                carousel_blurred_overlay::get_private(composition_source_id, author_id)
            }
            CarouselType::Images => carousel_images::get_private(composition_source_id, author_id),
        }
    }

    fn create(
        &self,
        composition_type: CarouselType,
        create_request: Box<dyn Any>,
        layout_id: u128,
        author_id: u128,
    ) {
        match composition_type {
            CarouselType::Basic => match create_request.downcast_ref::<CarouselBasicCreateReq>() {
                Some(req) => carousel_basic::create(req, layout_id, author_id),
                None => panic!("failed..."),
            },
            CarouselType::BlurredOverlay => {
                match create_request.downcast_ref::<CarouselBlurredOverlayCreateReq>() {
                    Some(req) => carousel_blurred_overlay::create(req, layout_id, author_id),
                    None => panic!("failed..."),
                }
            }
            CarouselType::Images => {
                match create_request.downcast_ref::<CarouselOfImagesCreateReq>() {
                    Some(req) => carousel_images::create(req, layout_id, author_id),
                    None => panic!("failed..."),
                }
            }
        }
    }

    fn update(
        &self,
        composition_type: CarouselType,
        composition_update_que: Vec<UpdateDataOfComposition>,
        composition_source_id: u128,
        author_id: u128,
    ) {
        match composition_type {
            CarouselType::Basic => {
                carousel_basic::update(composition_update_que, composition_source_id, author_id)
            }
            CarouselType::BlurredOverlay => carousel_blurred_overlay::update(
                composition_update_que,
                composition_source_id,
                author_id,
            ),
            CarouselType::Images => {
                carousel_images::update(composition_update_que, composition_source_id, author_id)
            }
        }
    }

    fn delete(&self, composition_type: CarouselType, composition_source_id: u128, author_id: u128) {
        match composition_type {
            CarouselType::Basic => carousel_basic::delete(composition_source_id, author_id),
            CarouselType::BlurredOverlay => {
                carousel_blurred_overlay::delete(composition_source_id, author_id)
            }
            CarouselType::Images => carousel_images::delete(composition_source_id, author_id),
        }
    }
}
