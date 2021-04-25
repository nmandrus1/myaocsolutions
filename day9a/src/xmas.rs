#[derive(Debug)]
pub struct Xmas {
    nums: Vec<usize>,
    nums_slice: Vec<usize>,
    nums_slice_len: usize,
    idx: usize,
}

impl Xmas {
    pub fn new(input: &str, preamble_len: usize) -> Xmas {
        Xmas {
            nums: input.lines().map(|i| i.parse::<usize>().unwrap()).collect(),
            nums_slice: input
                .lines()
                .take(preamble_len)
                .map(|i| i.parse::<usize>().unwrap())
                .collect(),
            nums_slice_len: preamble_len,
            idx: 0,
        }
    }

    pub fn next_num(&mut self) -> usize {
        let test_num = self.nums[self.idx + self.nums_slice_len];

        for i in 0..self.nums_slice_len {
            for j in (i + 1)..self.nums_slice_len {
                if self.nums_slice[i] + self.nums_slice[j] == test_num {
                    self.idx += 1;
                    self.new_nums_slice(test_num);
                    return 0;
                }
            }
        }

        test_num
    }

    fn new_nums_slice(&mut self, new_num: usize) {
        self.nums_slice.push(new_num);
        self.nums_slice.drain(0..1);
    }
}
