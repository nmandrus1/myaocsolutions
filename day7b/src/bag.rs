// Create Bag type that holds the type of bag and a
// Vec of tuples where each tuple contains and bag and
// a number where the number is the number of bags

#[derive(Debug, Clone)]
pub struct Bag {
    pub name: String,
    pub holds_total: usize,
    pub holds_in_vec: usize,
    pub bags_vec: Vec<(usize, Bag)>,
}

impl PartialEq for Bag {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Bag {
    pub fn shiny_gold() -> Bag {
        Bag {
            name: "shiny gold".to_string(),
            bags_vec: Vec::new(),
            holds_in_vec: 0,
            holds_total: 0,
        }
    }

    pub fn count_bags(&self, mut total: usize) -> usize {
        self.holds_total
    }

    pub fn populate_vec(&mut self, input: &str) {
        println!("Populating {}'s Vector...", self.name);

        let name = self.name.clone();

        input
            .lines()
            .filter(|line| line.starts_with(&name))
            .for_each(|string| {
                if string.contains("no") {
                    self.bags_vec = vec![];
                } else {
                    self.bags_vec = parse_input(string);
                    self.holds_in_vec = self.bags_vec.iter().map(|(i, _)| i).sum();
                    self.holds_total = 0;
                }
            });
    }
}

fn parse_input(line: &str) -> Vec<(usize, Bag)> {
    println!("Parsing Input\n");

    line.split("contain ")
        .skip(1)
        .flat_map(|s| s.split(", "))
        .map(|s| {
            let string = s.rsplit_once(' ').unwrap().0;
            (
                string[0..1].parse::<usize>().unwrap(),
                Bag {
                    name: string[2..string.len()].to_string(),
                    bags_vec: Vec::new(),
                    holds_in_vec: 0,
                    holds_total: 0,
                },
            )
        })
        .collect::<Vec<(usize, Bag)>>()
}
