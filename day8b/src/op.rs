// Enum that holds each possible operations
// and any argument associated with it
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Op {
    // Change the index by the amount specified
    // by the jump operation
    pub fn new_idx(idx: usize, arg: i32) -> usize {
        if arg.is_negative() {
            idx - arg.abs() as usize
        } else {
            idx + arg as usize
        }
    }
}

pub fn determine_op(op: &str, arg: &str) -> Op {
    if op.contains("acc") {
        return Op::Acc(arg.parse().expect("Failed to parse input"));
    } else if op.contains("jmp") {
        return Op::Jmp(arg.parse().expect("Failed to parse input"));
    } else if op.contains("nop") {
        return Op::Nop(arg.parse().expect("Failed to parse input"));
    } else {
        panic!("FAILED TO PARSE ARG");
    }
}
