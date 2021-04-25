mod bag;

use bag::Bag;

fn main() {
    let input = include_str!("input.txt");

    let mut sg = Bag::shiny_gold();
    sg.populate_vec(input);

    iter_through_bags_ref(&mut sg, input);

    println!("{:#?}\n\n\n", sg);
    println!("{}", sg.holds_total);
}

fn iter_through_bags_ref(bag: &mut Bag, input: &str) {
    for (i, bag_in_vec) in bag.bags_vec.iter_mut() {
        bag_in_vec.populate_vec(input);
        iter_through_bags_ref(bag_in_vec, input);

        bag.holds_total += (*i + (*i * bag_in_vec.holds_total));
    }
}

// Backup
//fn iter_through_bags_ref(bag: &mut Bag, input: &str) {
//for (_i, bag_in_vec) in bag.bags_vec.iter_mut() {
//bag_in_vec.populate_vec(input);
//iter_through_bags_ref(bag_in_vec, input)
//}
//}
