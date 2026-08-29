//! <<<description>>>
//!
//! <<<name>>> - <<<version>>>

??? >>> DEBUG
// This template's dependencies point to local paths,
// for debugging purposes only.
??? <<<

use mingling::macros::help;
use mingling::prelude::*;
use mingling::setup::picker::BasicProgramSetup;

fn main() {
    let mut program = ThisProgram::new();
    program.with_setup(BasicProgramSetup);
    program.exec_and_exit();
}

#[derive(Grouped)]
struct ResultHello {
    name: String,
}

#[command(entry = EntryHello)]
fn hello(args: EntryHello) -> ResultHello {
    let name = args.pick_or(&arg![String], || "World".into()).unwrap();
    ResultHello { name }
}

#[renderer]
fn render_hello(r: ResultHello) -> String {
    format!("Hello, {}!\n", r.name)
}

#[help]
fn help_hello(_: EntryHello) -> String {
    format!("USAGE: <<<name>>> hello <NAME>\n")
}

gen_program!();
