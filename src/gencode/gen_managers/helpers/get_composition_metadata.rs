use crate::compositions::banners::banner_type::BannerType;
use crate::compositions::carousels::carousel_type::CarouselType;
use crate::compositions::CompositionCategory;
use crate::gencode::gen_managers::helpers::get_mod;

pub fn get_composition_metadata(composition_category: &CompositionCategory) -> (String, String) {
    let f = get_mod::get_mod(&CompositionCategory::Carousel(CarouselType::Basic));

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
