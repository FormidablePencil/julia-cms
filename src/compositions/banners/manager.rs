use super::banner_basic::BannerCreateRes;
use super::banner_type::BannerType;
use super::{banner_basic, UpdateDataOfComposition};
use crate::compositions::banners::banner_basic::BannerCreateReq;
use crate::compositions::manager_impl::CompositionTypeManager;
use std::any::Any;

pub struct BannerManager;

pub enum BannerResponse {
    Basic(Option<BannerCreateRes>),
}

impl CompositionTypeManager<BannerType, BannerCreateReq, BannerResponse> for BannerManager {
    fn get_public(
        &self,
        composition_type: BannerType,
        composition_source_id: u128,
    ) -> BannerResponse {
        match composition_type {
            BannerType::Basic => {
                BannerResponse::Basic(banner_basic::get_public(composition_source_id))
            }
        }
    }

    fn get_private(
        &self,
        composition_type: BannerType,
        composition_source_id: u128,
        author_id: u128,
    ) -> BannerResponse {
        match composition_type {
            BannerType::Basic => {
                BannerResponse::Basic(banner_basic::get_private(composition_source_id, author_id))
            }
        }
    }

    fn create(
        &self,
        composition_type: BannerType,
        create_request: Box<dyn Any>,
        layout_id: u128,
        author_id: u128,
    ) -> Option<u128> {
        match composition_type {
            BannerType::Basic => match create_request.downcast_ref::<BannerCreateReq>() {
                Some(req) => banner_basic::create(req, layout_id, author_id),
                None => panic!("failed..."),
            },
        }
    }

    fn update(
        &self,
        composition_type: BannerType,
        composition_update_que: Vec<UpdateDataOfComposition>,
        composition_source_id: u128,
        author_id: u128,
    ) -> bool {
        match composition_type {
            BannerType::Basic => {
                banner_basic::update(composition_update_que, composition_source_id, author_id)
            }
        }
    }

    fn delete(
        &self,
        composition_type: BannerType,
        composition_source_id: u128,
        author_id: u128,
    ) -> bool {
        match composition_type {
            BannerType::Basic => banner_basic::delete(composition_source_id, author_id),
        }
    }
}
