use crate::compositions::UpdateDataOfComposition;

pub struct TextBasicCreateReq {}
pub struct TextBasicRes {}

pub fn get_public(composition_source_id: u128) -> Option<TextBasicRes> {
    todo!()
}

pub fn get_private(composition_source_id: u128, author_id: u128) -> Option<TextBasicRes> {
    todo!()
}

pub fn create(create_request: TextBasicCreateReq, layout_id: u128, author_id: u128) -> bool {
    todo!()
}

pub fn update(
    composition_update_que: Vec<UpdateDataOfComposition>,
    composition_source_id: u128,
    author_id: u128,
) -> bool {
    todo!()
}

pub fn delete(composition_source_id: u128, author_id: u128) -> bool {
    todo!()
}
