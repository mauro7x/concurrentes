// Notice how we can use our private bar:
use crate::part1::bar;

// But try to use private bar from part2:
// use crate::part2::bar;

pub fn run() -> () {
    println!("(part1:foo) Calling bar...");
    bar::run();
}
