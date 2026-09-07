//! <<<description>>>
//!
//! <<<name>>> - <<<version>>>

??? >>> DEBUG
// This template's dependencies point to local paths,
// for debugging purposes only.
??? <<<

use mingling::macros::{buffer, help};
use mingling::prelude::*;
use mingling::res::ResExitCode;
use mingling::setup::DefaultSetup;
??? >>> USE_ASYNC_STD_RT
use async_std::task::block_on;
??? <<<
??? >>> USE_SMOL_RT
use smol::block_on;
??? <<<

pub(crate) mod app;

??? >>> USE_TOKIO_RT
#[tokio::main]
async fn main() {
??? <<<
??? >>> USE_ASYNC_STD_RT
fn main() {
    block_on(begin());
}

async fn begin() {
??? <<<
??? >>> USE_SMOL_RT
fn main() {
    block_on(begin());
}

async fn begin() {
??? <<<
??? >>> USE_SYNC
fn main() {
??? <<<
    let mut program = ThisProgram::new();
    program.with_setup(DefaultSetup);
??? >>> USE_ASYNC
    program.exec_and_exit().await;
??? <<<
??? >>> USE_SYNC
    program.exec_and_exit();
??? <<<
}

#[help]
pub fn help(_: EntryFallback) -> String {
    include_str!("./help/program.txt").into()
}

#[renderer(buffer)]
pub fn render_fallback(input: EntryFallback, ec: &mut ResExitCode) {
    r_eprintln!("\"{}\" is not a valid input", input.0.join(", "));
    ec.exit_code = 1;
}

gen_program!();
