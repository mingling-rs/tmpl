use mingling::{hook::ProgramHook, macros::program_setup, Program};

use crate::ThisProgram;

fn build_<<<snake_case>>>_hook() -> ProgramHook<ThisProgram> {
    ProgramHook::empty().on_begin::<_, ()>(|_| {
        // ...
    })
}

#[program_setup]
pub fn <<<snake_case>>>_hook_setup(program: &mut Program<ThisProgram>) {
    let hook = build_<<<snake_case>>>_hook();
    program.with_hook(hook);
}
