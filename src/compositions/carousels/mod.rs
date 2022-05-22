use std::any::Any;

use super::{UpdateDataOfComposition, manager_impl::CompositionTypeManager};

use self::{
    carousel_basic::CarouselBasicCreateReq,
    carousel_blurred_overlay::CarouselBlurredOverlayCreateReq,
    carousel_images::CarouselOfImagesCreateReq, carousel_type::CarouselType,
    manager::CarouselResponse,
};

pub mod carousel_basic;
pub mod carousel_blurred_overlay;
pub mod carousel_images;
pub mod carousel_type;

pub mod manager;

pub struct CarouselManager {}

impl CompositionTypeManager<CarouselType, CarouselBasicCreateReq, CarouselResponse>
    for CarouselManager
{
    fn get_public(
        &self,
        composition_type: CarouselType,
        composition_source_id: u128,
    ) -> CarouselResponse {
        match composition_type {
            CarouselType::Basic => {
                CarouselResponse::CarouselBasic(carousel_basic::get_public(composition_source_id))
            }
            CarouselType::BlurredOverlay => CarouselResponse::CarouselBlurredOverlay(
                carousel_blurred_overlay::get_public(composition_source_id),
            ),
            CarouselType::Images => CarouselResponse::CarouselOfImages(
                carousel_images::get_public(composition_source_id),
            ),
        }
    }

    fn get_private(
        &self,
        composition_type: CarouselType,
        composition_source_id: u128,
        author_id: u128,
    ) -> CarouselResponse {
        match composition_type {
            CarouselType::Basic => CarouselResponse::CarouselBasic(carousel_basic::get_private(
                composition_source_id,
                author_id,
            )),
            CarouselType::BlurredOverlay => CarouselResponse::CarouselBlurredOverlay(
                carousel_blurred_overlay::get_private(composition_source_id, author_id),
            ),
            CarouselType::Images => CarouselResponse::CarouselOfImages(
                carousel_images::get_private(composition_source_id, author_id),
            ),
        }
    }

    fn create(
        &self,
        composition_type: CarouselType,
        create_request: Box<dyn Any>,
        layout_id: u128,
        author_id: u128,
    ) -> Option<u128> {
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
    ) -> bool {
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

    fn delete(
        &self,
        composition_type: CarouselType,
        composition_source_id: u128,
        author_id: u128,
    ) -> bool {
        match composition_type {
            CarouselType::Basic => carousel_basic::delete(composition_source_id, author_id),
            CarouselType::BlurredOverlay => {
                carousel_blurred_overlay::delete(composition_source_id, author_id)
            }
            CarouselType::Images => carousel_images::delete(composition_source_id, author_id),
        }
    }
}
