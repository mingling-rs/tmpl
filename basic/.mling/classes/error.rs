use mingling::{macros::renderer, Grouped};
??? >>> EXIT_CODE
use mingling::res::ResExitCode;
??? <<<

??? >>> EXIT_CODE
// Fill in the exit code for this error
const EC_ERROR_<<<upper_snake_case>>>: i32 = 0;
??? <<<

#[derive(Debug, Clone, Grouped, PartialEq, Eq)]
pub struct Error<<<pascal_case>>> {
    pub error_info: String,
}

??? >>> EXIT_CODE
#[renderer]
pub fn handle_error_<<<snake_case>>>(_error: Error<<<pascal_case>>>, ec: &mut ResExitCode) {
    // Update exit code to `EC_ERROR_<<<upper_snake_case>>>`
    ec.exit_code = EC_ERROR_<<<upper_snake_case>>>;

    // TODO: Implement handling logic for error `Error<<<pascal_case>>>`
    todo!()
}
??? <<<
??? >>> NOT_EXIT_CODE
#[renderer]
pub fn handle_error_<<<snake_case>>>(_error: Error<<<pascal_case>>>) {
    // TODO: Implement handling logic for error `Error<<<pascal_case>>>`
    todo!()
}
??? <<<
