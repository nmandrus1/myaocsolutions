mod program;
use program::Program;

mod op;
pub use op::Op;

fn main() {
    //let input = include_str!("input_test.txt");
    let input = include_str!("input.txt");

    let mut program = Program::new_from_input(input);
    program.run();
    let v = program.ops_execed.clone();

    for idx in v {
        program.switch_op(idx);
        program.run();

        if program.good_exit {
            println!("{}", program.acc);
        }

        program.switch_op(idx);
    }
}
