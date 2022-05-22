use codegen::Scope;

use crate::{compositions::CompositionCategory, get_composition_name};

use super::{
    helpers::{
        get_composition_create_request, get_composition_name, get_composition_response_enum,
        import_composition_models,
    },
    import_mods::import_composition_mods,
};

pub fn manager_dependents(scope: &mut Scope, composition_category: CompositionCategory) {
    let composition_type = get_composition_name(&composition_category, true);
    let composition_name = get_composition_name(&composition_category, false);

    scope.import("std::any", "Any");
    scope.import(
        "crate::compositions::manager_impl",
        "CompositionTypeManager",
    );

    scope.import("strum_macros", "EnumIter");
    scope.import("strum_macros", "EnumString");
    // scope.import(
    //     format!("self::{}", get_mod(&composition_category)).as_str(),
    //     create_request,
    // );
    scope.import("super", "UpdateDataOfComposition");
    let comp_name = get_composition_name!(&composition_category).to_lowercase();
    import_composition_models(&mut scope, &composition_category);
    import_composition_mods(&mut scope, &composition_category);
    // scope.import("super", format!("{comp_name}_basic").as_str());
    // scope.import("super", "carousel_blurred_overlay");
    // scope.import("super", "carousel_images");

    // get_composition_create_request
    import_composition_models(&mut scope, &composition_category);
    scope.import(
        format!("super::{comp_name}_type").as_str(),
        composition_type.as_str(),
    );

    // scope
    //     .new_enum(format!("{composition_type}Type").as_str())
    //     .vis("pub")
    //     .derive("Debug")
    //     .derive("EnumIter")
    //     .derive("EnumString")
    //     .new_variant("Basic, SomeOtherComp");

    scope
        .new_struct(format!("{composition_name}Manager").as_str())
        .vis("pub");

    // todo - static data
    scope.import("banner_basic", "BannerRes");
    scope
        .new_enum(get_composition_response_enum(&composition_category).as_str())
        .vis("pub")
        // todo map through this
        .new_variant("Basic(Option<BannerRes>)");
    // todo - static data
}
