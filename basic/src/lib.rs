use mingling::prelude::*;

pub(crate) mod command;
??? >>> COMPLETION
pub(crate) mod completion;
??? <<<
pub(crate) mod error;
pub(crate) mod fallback;
pub(crate) mod help;
pub(crate) mod hook;
pub(crate) mod resource;
pub(crate) mod setup;

??? >>> NOT_DISPATCH_TREE
pub use setup::commands_setup::CommandsSetup;
??? <<<

gen_program!();
