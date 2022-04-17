use crate::compositions::UpdateDataOfComposition;

#[derive(Debug)]
pub struct BannerCreateReq {}

pub fn get_public(composition_source_id: u128) {}
pub fn get_private(composition_source_id: u128, author_id: u128) {}
pub fn create(create_request: BannerCreateReq, layout_id: u128, author_id: u128) {}
pub fn update(
    composition_update_que: Vec<UpdateDataOfComposition>,
    composition_source_id: u128,
    author_id: u128,
) {
}
pub fn delete(composition_source_id: u128, author_id: u128) {}
