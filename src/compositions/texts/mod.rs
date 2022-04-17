pub mod text_basic;

use crate::compositions::UpdateDataOfComposition;

use self::text_basic::TextBasicCreateReq;

pub trait CompositionTypeManager<CompositionType, CreateRequest> {
    fn get_public(&self, comp_type: CompositionType, composition_source_id: u128);

    fn get_private(&self, comp_type: CompositionType, composition_source_id: u128, author_id: u128);

    fn create(
        &self,
        comp_type: CompositionType,
        create_request: CreateRequest,
        layout_id: u128,
        author_id: u128,
    );

    fn update(
        &self,
        comp_type: CompositionType,
        composition_update_que: Vec<UpdateDataOfComposition>,
        layout_id: u128,
        author_id: u128,
    );

    fn delete(&self, comp_type: CompositionType, composition_source_id: u128, author_id: u128);
}

pub struct TextManager {}

pub enum TextType {
    TextBasic,
}

impl CompositionTypeManager<TextType, TextBasicCreateReq> for TextManager {
    // pub fn new() -> Self {
    //     Self {}
    // }

    fn get_public(&self, comp_type: TextType, composition_source_id: u128) {
        match comp_type {
            TextType::TextBasic => text_basic::get_public(composition_source_id),
        }
    }

    fn get_private(&self, comp_type: TextType, composition_source_id: u128, author_id: u128) {
        match comp_type {
            TextType::TextBasic => text_basic::get_private(composition_source_id, author_id),
        }
    }

    fn create(
        &self,
        comp_type: TextType,
        create_request: TextBasicCreateReq,
        layout_id: u128,
        author_id: u128,
    ) {
        match comp_type {
            TextType::TextBasic => text_basic::create(create_request, layout_id, author_id),
        }
    }

    fn update(
        &self,
        comp_type: TextType,
        composition_update_que: Vec<UpdateDataOfComposition>,
        layout_id: u128,
        author_id: u128,
    ) {
        match comp_type {
            TextType::TextBasic => text_basic::update(composition_update_que, layout_id, author_id),
        }
    }

    fn delete(&self, comp_type: TextType, composition_source_id: u128, author_id: u128) {
        match comp_type {
            TextType::TextBasic => text_basic::delete(composition_source_id, author_id),
        }
    }
}
