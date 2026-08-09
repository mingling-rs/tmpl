use crate::*;
use mingling::{
??? >>> COMPLETION
    ShellContext, Suggest,
??? <<<
    macros::{command, help, metadata},
    metadata::Description,
};

#[help]
pub fn help_<<<snake_case>>>(_: Entry<<<pascal_case>>>) -> String {
    "Usage: <<<snake_case>>> <ARGS...>".to_string()
}

#[metadata(Entry<<<pascal_case>>>)]
pub fn desc_<<<snake_case>>>() -> Description {
    "subcommand '<<<snake_case>>>'".into()
}

#[command]
pub fn <<<snake_case>>>(_args: Entry) {
    // TODO: Implement the behavior of the `<<<snake_case>>>` command
    todo!()
}

??? >>> COMPLETION
#[completion(Entry<<<pascal_case>>>)]
pub fn complete_<<<snake_case>>>(_ctx: &ShellContext) -> Suggest {
    Suggest::FileCompletion
}
??? <<<
