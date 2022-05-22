use super::UpdateDataOfComposition;

pub mod banner_basic;
pub mod manager;
pub mod banner_type;

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
