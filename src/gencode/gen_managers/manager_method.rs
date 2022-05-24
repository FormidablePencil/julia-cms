use strum_macros::EnumIter;

use crate::compositions::CompositionCategory;
use crate::gencode::gen_managers::helpers::get_composition_name::get_composition_name;
use crate::gencode::gen_managers::helpers::get_mod::get_mod;

#[derive(Debug, EnumIter)]
pub enum ManagerMethod {
    GetPublic,
    GetPrivate,
    Create,
    Update,
    Delete,
}

impl ManagerMethod {
    pub(crate) fn get_method_name(&self) -> String {
        match self {
            ManagerMethod::GetPublic => "get_public".to_string(),
            ManagerMethod::GetPrivate => "get_private".to_string(),
            ManagerMethod::Create => "create".to_string(),
            ManagerMethod::Update => "update".to_string(),
            ManagerMethod::Delete => "delete".to_string(),
        }
    }

    pub(crate) fn get_args_for_method(&self) -> String {
        match self {
            ManagerMethod::GetPublic => "(composition_source_id)".to_string(),
            ManagerMethod::GetPrivate => "(composition_source_id, author_id)".to_string(),
            ManagerMethod::Create => "(create_request, layout_id, author_id)".to_string(),
            ManagerMethod::Update => "(composition_update_que, composition_source_id, author_id)".to_string(),
            ManagerMethod::Delete => "(composition_source_id, author_id)".to_string(),
        }
    }

    // todo - obsolete code
    pub fn get_arm(
        &self,
        composition_category: &CompositionCategory,
        enum_type_name: String,
        update_or_delete: bool,
    ) -> String {
        let arm_left = format!(
            "{}::{enum_type_name}",
            get_composition_name(&composition_category, true),
        );

        let arm_right = if update_or_delete == true {
            format!(
                "{}Response::{enum_type_name}({}::{}{}),",
                get_composition_name(&composition_category, false),
                get_mod(&composition_category),
                self.get_method_name(),
                self.get_args_for_method()
            )
        } else {
            format!(
                "{}::{}{},",
                get_mod(&composition_category),
                self.get_method_name(),
                self.get_args_for_method()
            )
        };
        // self.push_block(
        //     Block::new(matcher.as_str())
        //         .line(format!(
        //             "Some(req) => {}::create(req, layout_id, author_id),",
        //             get_mod(&composition_category)
        //         ))
        //         .line("None => panic!(\"failed...\")")
        //         .to_owned(),
        // )

        format!("{arm_left} => {arm_right}")
    }

    fn get_arm_for_create() {}
}
