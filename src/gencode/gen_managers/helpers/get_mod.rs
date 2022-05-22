use crate::compositions::banners::banner_type::BannerType;
use crate::compositions::carousels::carousel_type::CarouselType;
use crate::compositions::CompositionCategory;
use crate::compositions::texts::manager::TextType;

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
            TextType::Basic => "text_basic".to_string(),
        },
    }
}
