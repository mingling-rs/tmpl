use mingling::{Program, ProgramCollect, setup::ProgramSetup};

/// `<<<name>>>` program setup - <<<pascal_case>>>Setup
pub(crate) struct <<<pascal_case>>>Setup;

impl<ThisProgram> ProgramSetup<ThisProgram> for <<<pascal_case>>>Setup
where
    ThisProgram: ProgramCollect<Enum = ThisProgram>,
{
    fn setup(self, program: &mut Program<ThisProgram>) {
        // TODO:: Impl modification behavior on program
    }
}
