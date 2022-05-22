use std::any::Any;

use strum_macros::{EnumIter, EnumString};

use crate::compositions::{manager_impl::CompositionTypeManager, UpdateDataOfComposition};

use super::text_basic::{TextBasicCreateReq, self};

pub struct TextManager {}

#[derive(Debug, EnumIter, EnumString)]
pub enum TextType {
    Basic,
}

impl CompositionTypeManager<TextType, TextBasicCreateReq> for TextManager {
    // pub fn new() -> Self {
    //     Self {}
    // }

    fn get_public(&self, comp_type: TextType, composition_source_id: u128) {
        match comp_type {
            TextType::Basic => text_basic::get_public(composition_source_id),
        }
    }

    fn get_private(&self, comp_type: TextType, composition_source_id: u128, author_id: u128) {
        match comp_type {
            TextType::Basic => text_basic::get_private(composition_source_id, author_id),
        }
    }

    fn create(
        &self,
        comp_type: TextType,
        create_request: Box<dyn Any>,
        layout_id: u128,
        author_id: u128,
    ) {
        // match comp_type {
        //     TextType::Basic => text_basic::create(create_request, layout_id, author_id),
        // }
    }

    fn update(
        &self,
        comp_type: TextType,
        composition_update_que: Vec<UpdateDataOfComposition>,
        layout_id: u128,
        author_id: u128,
    ) {
        match comp_type {
            TextType::Basic => text_basic::update(composition_update_que, layout_id, author_id),
        }
    }

    fn delete(&self, comp_type: TextType, composition_source_id: u128, author_id: u128) {
        match comp_type {
            TextType::Basic => text_basic::delete(composition_source_id, author_id),
        }
    }
}