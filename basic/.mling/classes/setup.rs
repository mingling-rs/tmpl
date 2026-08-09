use mingling::{macros::program_setup, Program};

use crate::ThisProgram;

#[program_setup]
pub fn <<<snake_case>>>_setup(_program: &mut Program<ThisProgram>) {
    // Implement the modification logic for the Program
}
