use mingling::{ShellContext, Suggest, macros::help, prelude::*};

#[command(entry = Entry<<<pascal_case>>>)]
pub fn <<<snake_case>>>(args: Entry<<<pascal_case>>>) {
    // TODO:: Impl the behavior of the `<<<subcommand_case>>>` command
}

#[completion(Entry<<<pascal_case>>>)]
pub fn complete_<<<snake_case>>>(ctx: ShellContext) -> Suggest {
    // TODO:: Impl completion for the `<<<subcommand_case>>>` command
    Suggest::file_comp()
}

#[help]
pub fn usage_<<<snake_case>>>(_: Entry<<<pascal_case>>>) -> String {
    r"
Usage: <<<name>>> <<<subcommand_case>>> <ARG...>
"
    .trim()
    .to_string()
}
