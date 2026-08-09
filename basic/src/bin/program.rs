??? >>> NOT_PARSER_PICKER
use mingling::setup::BasicProgramSetup;
??? <<<
??? >>> EXIT_CODE
use mingling::setup::ExitCodeSetup;
??? <<<
??? >>> PARSER_PICKER
use mingling::setup::picker::BasicProgramSetup;
??? <<<
use <<<program_crate_name>>>::*;

??? >>> SYNC
fn main() {
??? <<<
??? >>> TOKIO
#[tokio::main]
async fn main() {
??? <<<
??? >>> ASYNC_STD
fn main() {
    async_std::task::block_on(begin());
}

async fn begin() {
??? <<<
??? >>> SMOL
fn main() {
    smol::block_on(begin());
}

async fn begin() {
??? <<<
    let mut program = ThisProgram::new();

    // Plugins
    program.with_setup(BasicProgramSetup);
??? >>> EXIT_CODE
    program.with_setup(ExitCodeSetup::default());
??? <<<
??? >>> COMPLETION

    // Completion Dispatcher
    program.with_dispatcher(CMDCompletion);
??? <<<

??? >>> SYNC
    program.exec_and_exit();
??? <<<
??? >>> ASYNC
    program.exec_and_exit().await;
??? <<<
}
