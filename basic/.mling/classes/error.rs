use mingling::{macros::renderer, Grouped};

#[derive(Debug, Clone, Grouped, PartialEq, Eq)]
pub struct Error<<<pascal_case>>> {
    pub error_info: String,
}

#[renderer]
pub fn handle_error_<<<snake_case>>>(_error: Error<<<pascal_case>>>) {
    // TODO: Implement handling logic for error `Error<<<pascal_case>>>`
    todo!()
}
