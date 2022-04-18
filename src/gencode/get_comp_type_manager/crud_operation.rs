use strum_macros::EnumIter;

#[derive(Debug, EnumIter)]
pub enum CrudOperation {
    GetPublic,
    GetPrivate,
    Create,
    Update,
    Delete,
}

impl CrudOperation {
    fn get_method_name(&self) -> String {
        match self {
            CrudOperation::GetPublic => "get_public".to_string(),
            CrudOperation::GetPrivate => "get_private".to_string(),
            CrudOperation::Create => "create".to_string(),
            CrudOperation::Update => "update".to_string(),
            CrudOperation::Delete => "delete".to_string(),
        }
    }

    fn get_args_for_method(&self) -> String {
        match self {
            CrudOperation::GetPublic => "(composition_source_id)".to_string(),
            CrudOperation::GetPrivate => "(composition_source_id, author_id)".to_string(),
            CrudOperation::Create => "(create_request, layout_id, author_id)".to_string(),
            CrudOperation::Update => {
                "(composition_update_que, composition_source_id, author_id)".to_string()
            }
            CrudOperation::Delete => "(composition_source_id, author_id)".to_string(),
        }
    }

    pub fn get_arm(
        &self,
        enum_name: String,
        enum_type_name: String,
        comp_type_struct_name: String,
    ) -> String {
        let arm_left = format!("{}::{} =>", enum_name, enum_type_name);
        let arm_right = format!(
            "{}::{}{},",
            comp_type_struct_name,
            self.get_method_name(),
            self.get_args_for_method()
        );

        format!("{} {}", arm_left, arm_right)
    }

    fn get_arm_for_create() {}
}
