pub mod compositions;
pub mod gencode;

// use compositions::carousels::carousel_blurred_overlay;
// use ::CarouselDT;

#[cfg(test)]
mod carousel {
    use crate::compositions::carousels::carousel_blurred_overlay::get_public;

    #[test]
    fn add() {
        // call logic of banners model directly
        get_public(1)
        // assert_eq!(2 + 2, 4);
    }

    #[test]
    fn delete() {
        // panic!("Make this test fail");
    }
}
