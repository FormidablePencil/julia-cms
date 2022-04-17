use strum_macros::{EnumIter, EnumString};

use self::banner_basic::BannerCreateReq;

use super::{texts::CompositionTypeManager, UpdateDataOfComposition};

pub mod banner_basic;

#[derive(Debug, EnumIter, EnumString)]
pub enum BannerType {
    Basic,
    SomeOtherComp,
}

// move to separate file
// impl std::fmt::Display for BannerType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         // write!(f, "{}", self)
//         write!(f, "{}", self)
//     }
// }

pub struct BannerManager {}

// impl CompositionTypeManager<BannerType, BannerCreateReq> for BannerManager {
//     // pub fn new() -> Self {
//     //     Self {}
//     // }

//     fn get_public(&self, comp_type: BannerType, composition_source_id: u128) {
//         match comp_type {
//             BannerType::Basic => banner_basic::get_public(composition_source_id),
//         }
//     }

//     fn get_private(&self, comp_type: BannerType, composition_source_id: u128, author_id: u128) {
//         match comp_type {
//             BannerType::Basic => banner_basic::get_private(composition_source_id, author_id),
//         }
//     }

//     fn create(
//         &self,
//         comp_type: BannerType,
//         create_request: BannerCreateReq,
//         layout_id: u128,
//         author_id: u128,
//     ) {
//         match comp_type {
//             BannerType::Basic => banner_basic::create(create_request, layout_id, author_id),
//         }
//     }

//     fn update(
//         &self,
//         comp_type: BannerType,
//         composition_update_que: Vec<UpdateDataOfComposition>,
//         composition_source_id: u128,
//         author_id: u128,
//     ) {
//         match comp_type {
//             BannerType::Basic => {
//                 banner_basic::update(composition_update_que, composition_source_id, author_id)
//             }
//         }
//     }

//     fn delete(&self, comp_type: BannerType, composition_source_id: u128, author_id: u128) {
//         match comp_type {
//             BannerType::Basic => banner_basic::delete(composition_source_id, author_id),
//         }
//     }
// }
