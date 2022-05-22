use crate::compositions::CompositionCategory;
use crate::gencode::gen_managers::helpers::{get_composition_name, get_composition_res_enum};
use crate::gencode::gen_managers::helpers::iterate_over_all_enums_of_composition_category::iterate_over_all_enums_of_composition_category;

pub fn get_all_composition_res_variants_enums(
    composition_category: &CompositionCategory,
) -> String {
    let mut composition_enums_variants = String::from("");

    iterate_over_all_enums_of_composition_category(
        composition_category,
        &mut |comp_category: CompositionCategory| {
            composition_enums_variants.push_str(&get_composition_res_enum::get_composition_res_enum(composition_category));
        },
    );

    composition_enums_variants
}

pub fn get_composition_response_enum(composition_category: &CompositionCategory) -> String {
    format!(
        "{}Response",
        get_composition_name::get_composition_name(composition_category, false)
    )
}
