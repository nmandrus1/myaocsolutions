use super::op::{determine_op, Op};

#[derive(Debug)]
pub struct Program {
    pub ops: Vec<Op>,
    pub ops_execed: Vec<usize>,
    pub acc: i32,
    idx: usize,
    pub good_exit: bool,
}

impl Program {
    pub fn new_from_input(input: &str) -> Program {
        Program {
            ops: input_to_vec(input),
            ops_execed: Vec::new(),
            acc: 0,
            idx: 0,
            good_exit: true,
        }
    }

    pub fn switch_op(&mut self, idx: usize) {
        match self.ops[idx] {
            Op::Jmp(arg) => self.ops[idx] = Op::Nop(arg),
            Op::Nop(arg) => self.ops[idx] = Op::Jmp(arg),
            _ => (),
        }
    }

    fn reset_program(&mut self) {
        self.acc = 0;
        self.idx = 0;
        self.good_exit = true;
    }

    pub fn run(&mut self) {
        let mut tracking_vec = vec![false; self.ops.len()];
        self.reset_program();

        while self.idx != self.ops.len() {
            if tracking_vec[self.idx] {
                self.good_exit = false;
                break;
            }
            tracking_vec[self.idx] = true;
            self.ops_execed.push(self.idx);

            match self.ops[self.idx] {
                Op::Acc(arg) => {
                    self.acc += arg;
                    self.idx += 1;
                }
                Op::Jmp(arg) => self.idx = Op::new_idx(self.idx, arg),
                Op::Nop(_arg) => self.idx += 1,
            }
        }
    }
}

// Iterates through the input and creates vector of operations
fn input_to_vec(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            determine_op(iter.next().unwrap(), iter.next().unwrap())
        })
        .collect()
}
