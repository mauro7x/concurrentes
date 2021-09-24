// Notice how we can use our private bar:
use crate::part2::bar;

// But try to use private bar from part1:
// use crate::part1::bar;

pub fn run() {
    println!("(part2:foo) Calling bar...");
    bar::run();
}
