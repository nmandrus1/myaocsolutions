#[derive(Debug)]
enum OpKind {
    Acc,
    Jmp,
    Nop,
}

#[derive(Debug)]
struct Op {
    kind: OpKind,
    arg: isize,
}

fn main() {
    let input = include_str!("input.txt");

    let op_vec = input_to_vec(input);

    // All initially false and set to
    // true when corresponding op is run
    let mut true_false_vec = vec![false; op_vec.len()];

    let mut acc = 0;
    let mut idx = 0;

    loop {
        if true_false_vec[idx] {
            break;
        } else {
            true_false_vec[idx] = true
        }

        match op_vec[idx].kind {
            OpKind::Acc => {
                acc += op_vec[idx].arg;
                idx += 1;
            }
            OpKind::Jmp => {
                idx = get_idx_from_jmp(idx, op_vec[idx].arg);
            }
            OpKind::Nop => {
                idx += 1;
            }
        }

        println!("{:#?} \n\n {} \n\n {}", true_false_vec, acc, idx);
    }

    println!("{}", acc);
}

fn get_idx_from_jmp(idx: usize, arg: isize) -> usize {
    if arg.is_negative() {
        idx - arg.abs() as usize
    } else {
        idx + arg as usize
    }
}

fn input_to_vec(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_ascii_whitespace();
            Op {
                kind: determine_OpKind(iter.next().unwrap()),
                arg: iter.next().unwrap().parse::<isize>().unwrap(),
            }
        })
        .collect()
}

fn determine_OpKind(op: &str) -> OpKind {
    if op.contains("acc") {
        return OpKind::Acc;
    } else if op.contains("jmp") {
        return OpKind::Jmp;
    } else if op.contains("nop") {
        return OpKind::Nop;
    } else {
        panic!("FAILED TO PARSE ARG");
    }
}
