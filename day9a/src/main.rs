mod xmas;
use xmas::Xmas;

fn main() {
    //let input = include_str!("input_test.txt");
    let input = include_str!("input.txt");

    let mut xmas = Xmas::new(input, 25);

    loop {
        let num = xmas.next_num();
        if num == 0 {
            continue;
        }
        println!("{}", num);
        break;
    }
}
