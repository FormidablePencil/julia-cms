use codegen::Scope;
use strum::IntoEnumIterator;

use crate::compositions::{
    banners::banner_type::BannerType, carousels::carousel_type::CarouselType, texts::TextType,
    CompositionCategory,
};

use super::helpers::get_mod;

pub fn import_composition_mods(scope: &mut Scope, composition_category: &CompositionCategory) {
    match composition_category {
        CompositionCategory::Carousel(_) => {
            for item in CarouselType::iter() {
                let mod_name = get_mod(&CompositionCategory::Carousel(item));
                scope.import("super", mod_name.as_str());
            }
        }
        CompositionCategory::Banner(_) => {
            for item in BannerType::iter() {
                let mod_name = get_mod(&CompositionCategory::Banner(item));
                scope.import("super", mod_name.as_str());
            }
        }
        CompositionCategory::Text(_) => {
            for item in BannerType::iter() {
                let mod_name = get_mod(&CompositionCategory::Banner(item));
                scope.import("super", mod_name.as_str());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use codegen::Scope;

    use crate::compositions::{carousels::carousel_type::CarouselType, CompositionCategory};

    use super::import_composition_mods;

    #[test]
    fn import_composition_mods_test() {
        let mut scope = Scope::new();

        import_composition_mods(
            &mut scope,
            &CompositionCategory::Carousel(CarouselType::Basic),
        );

        print!("{}", scope.to_string());
    }
}
