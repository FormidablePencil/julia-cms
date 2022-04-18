use std::{fs::File, io::Write};

use codegen::{Block, Scope};
use strum::IntoEnumIterator;

use crate::compositions::{
    banners::BannerType, carousels::carousel_type::CarouselType, texts::TextType,
    CompositionCategory,
};

pub fn get_mod(composition_category: &CompositionCategory) -> String {
    match composition_category {
        CompositionCategory::Carousel(comp_type) => match comp_type {
            CarouselType::Basic => "carousel_basic".to_string(),
            CarouselType::BlurredOverlay => "carousel_blurred_overlay".to_string(),
            CarouselType::Images => "carousel_images".to_string(),
        },
        CompositionCategory::Banner(comp_type) => match comp_type {
            BannerType::Basic => "banner_basic".to_string(),
            BannerType::SomeOtherComp => "some_other_comp".to_string(),
        },
        CompositionCategory::Text(comp_type) => match comp_type {
            crate::compositions::texts::TextType::Basic => "text_basic".to_string(),
        },
    }
}

// todo - iterators with this pattern
pub fn import_composition_models(scope: &mut Scope, composition_category: &CompositionCategory) {
    iterate_over_all_enums_of_composition_category(
        composition_category,
        &mut |comp_category: CompositionCategory| {
            scope.import(format!("{:?}", comp_category).as_str(), "df");
        },
    );
}

// fn execute_closure<F>(mut closure_argument: F)
// where
//     F: FnMut() -> i32,
// {
//     let result = closure_argument();
//     println!("Result of closure: {}", result);
// }

// fn execute_closure(mut closure_argument: &dyn FnMut() -> i32) {
//     let result = closure_argument();
//     println!("Result of closure: {}", result);
// }

fn iterate_over_all_enums_of_composition_category(
    composition_category: &CompositionCategory,
    callback: &mut dyn FnMut(CompositionCategory),
) {
    match composition_category {
        CompositionCategory::Carousel(_) => {
            for item in CarouselType::iter() {
                callback(CompositionCategory::Carousel(item));
            }
        }
        CompositionCategory::Banner(_) => {
            for item in BannerType::iter() {
                callback(CompositionCategory::Banner(item));
                // banner_callback(item);
            }
        }
        CompositionCategory::Text(_) => {
            for item in TextType::iter() {
                callback(CompositionCategory::Text(item));
                // text_callback(item);
            }
        }
    }
}

pub fn get_composition_name(composition_category: &CompositionCategory, is_type: bool) -> String {
    match composition_category {
        CompositionCategory::Carousel(_) => "Carousel",
        CompositionCategory::Banner(_) => "Banner",
        CompositionCategory::Text(_) => "Text",
    }
    .to_string()
        + if is_type == true { "Type" } else { "" }
}

#[macro_export]
macro_rules! get_composition_name {
    ($composition_category: expr) => {
        get_composition_name($composition_category, false)
    }; // () => {
       //     get_composition_name(1, 2)
       // };
}
