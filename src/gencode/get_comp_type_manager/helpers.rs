use codegen::Scope;
use strum::IntoEnumIterator;

use crate::compositions::banners::banner_type::BannerType;
use crate::compositions::{
    carousels::carousel_type::CarouselType, texts::TextType, CompositionCategory,
};

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
            }
        }
        CompositionCategory::Text(_) => {
            for item in TextType::iter() {
                callback(CompositionCategory::Text(item));
            }
        }
    }
}

// todo - iterators with this pattern
pub fn import_composition_models(scope: &mut Scope, composition_category: &CompositionCategory) {
    iterate_over_all_enums_of_composition_category(
        composition_category,
        &mut |comp_category: CompositionCategory| {
            let (path, create_request) = get_composition_create_request(&comp_category);
            scope.import(path.as_str(), create_request.as_str());
        },
    );
}

pub fn get_composition_create_request(
    composition_category: &CompositionCategory,
) -> (String, String) {
    fn setup_import(
        composition_category: &CompositionCategory,
    ) -> Box<dyn FnOnce(String, String) -> (String, String)> {
        fn setup(
            composition_category: &CompositionCategory,
        ) -> Box<dyn FnOnce(String, String) -> (String, String)> {
            let setup_path = match composition_category {
                CompositionCategory::Carousel(_) => "crate::compositions::carousels",
                CompositionCategory::Banner(_) => "crate::compositions::banners",
                CompositionCategory::Text(_) => "crate::compositions::texts",
            }
            .to_string();

            Box::new(move |path: String, request: String| {
                (format!("{setup_path}::{path}"), String::from(request))
            })
        }

        match composition_category {
            CompositionCategory::Carousel(_) => setup(&composition_category),
            CompositionCategory::Banner(_) => setup(&composition_category),
            CompositionCategory::Text(_) => setup(&composition_category),
        }
    }

    let (first, second) = get_composition_metadata(&composition_category);
    let import = setup_import(&composition_category);
    import(first, second)
}

// todo - implement a cleaner func. Use array with enum instead
pub fn get_mod(composition_category: &CompositionCategory) -> String {
    match composition_category {
        CompositionCategory::Carousel(comp_type) => match comp_type {
            CarouselType::Basic => "carousel_basic".to_string(),
            CarouselType::BlurredOverlay => "carousel_blurred_overlay".to_string(),
            CarouselType::Images => "carousel_images".to_string(),
        },
        CompositionCategory::Banner(comp_type) => match comp_type {
            BannerType::Basic => "banner_basic".to_string(),
        },
        CompositionCategory::Text(comp_type) => match comp_type {
            crate::compositions::texts::TextType::Basic => "text_basic".to_string(),
        },
    }
}

pub fn get_composition_metadata(composition_category: &CompositionCategory) -> (String, String) {
    let f = get_mod(&CompositionCategory::Carousel(CarouselType::Basic));

    match composition_category {
        CompositionCategory::Carousel(comp_type) => match comp_type {
            CarouselType::Basic => (f, "CarouselBasicCreateReq".to_string()),
            CarouselType::BlurredOverlay => (
                "carousel_blurred_overlay".to_string(),
                "CarouselBlurredOverlayCreateReq".to_string(),
            ),
            CarouselType::Images => (
                "carousel_images".to_string(),
                "CarouselOfImagesCreateReq".to_string(),
            ),
        },
        CompositionCategory::Banner(comp_type) => match comp_type {
            BannerType::Basic => ("banner_basic".to_string(), "BannerCreateReq".to_string()),
        },
        CompositionCategory::Text(_) => todo!(),
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
    };
}

#[cfg(test)]
mod tests {
    use codegen::Scope;

    use crate::compositions::carousels::carousel_type::CarouselType;
    use crate::compositions::CompositionCategory;
    use crate::gencode::get_comp_type_manager::helpers::{
        get_composition_create_request, get_composition_metadata, import_composition_models,
    };

    #[test]
    fn get_composition_create_request_test() {
        let res =
            get_composition_create_request(&CompositionCategory::Carousel(CarouselType::Basic));
        print!("{}, {}", res.0, res.1);
    }

    #[test]
    fn get_composition_metadata_test() {
        let res = get_composition_metadata(&CompositionCategory::Carousel(CarouselType::Basic));
        assert_eq!(
            format!("{}, {}", res.0, res.1),
            "carousel_basic, CarouselBasicCreateReq"
        );
    }

    #[test]
    fn import_composition_models_test() {
        let mut scope = Scope::new();

        import_composition_models(
            &mut scope,
            &CompositionCategory::Carousel(CarouselType::Basic),
        );

        print!("{}", scope.to_string());
        // should print:
        // use crate::compositions::carousels::carousel_basic::CarouselBasicCreateReq;
        // use crate::compositions::carousels::carousel_blurred_overlay::CarouselBlurredOverlayCreateReq;
        // use crate::compositions::carousels::carousel_images::CarouselOfImagesCreateReq;
    }
}

// todo - tests

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
