use crate::EntryFallback;
use crate::ErrorRendererNotFound;

??? >>> COMPLETION
use mingling::ShellContext;
use mingling::Suggest;
??? <<<
use mingling::macros::buffer;
use mingling::macros::help;
??? >>> COMPLETION
use mingling::macros::suggest;
??? <<<
use mingling::prelude::*;

#[help]
pub(crate) fn help_fallback(prev: EntryFallback) -> String {
    include_str!("../usage.txt").to_string()
}

#[renderer(buffer)]
pub(crate) fn handle_fallback(prev: EntryFallback) {
    r_println!("Error: cannot match \"{}\" to any command", prev.join(" "));
}

??? >>> COMPLETION
#[completion(EntryFallback)]
pub(crate) fn complete_fallback(_ctx: &ShellContext) -> Suggest {
    suggest! {}
}
??? <<<

#[renderer(buffer)]
pub(crate) fn handle_error_renderer_not_found(prev: ErrorRendererNotFound) {
    let type_name = prev.inner;
    r_println!("Error: renderer not found for \"{}\"", type_name);
}
