use strum_macros::{EnumIter, EnumString};

#[derive(Debug, EnumIter, EnumString)]
pub enum CarouselType {
    Basic,
    BlurredOverlay,
    Images,
}
