use std::fs;
use std::ops::{Deref, DerefMut};

struct Range {
    start: u128,
    end: u128,
}

impl Range {

    fn contains(&self, value: u128) -> bool
    {
        // println!("Checking {} in {},{}", value, self.start, self.end);
        return self.start <= value && value <= self.end;
    }

}

struct Ranges(Vec<Range>);

impl Ranges {
    fn contains(&self, value: u128) -> bool
    {
        return self.0.iter().any(|r| {
            // println!("Checking {} in {},{}", value, r.start, r.end);
            r.contains(value)
            });
    }
}

impl Deref for Ranges {
    type Target = Vec<Range>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Ranges {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn main() {
    let mut contents = fs::read_to_string("./src/puzzle2.txt").expect("Cannot read the file");
    contents.pop();

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut ranges: Ranges = Ranges(Vec::new());
    let mut finished = false;
    let mut fresh_count = 0;

    for line in lines {
        if line.is_empty() { finished = true; continue; }

        if !finished {
            let split: Vec<u128> = line.split("-").map(|sn| sn.parse().unwrap()).collect();
            // println!("Adding range {};{}", split[0], split[1]);
            ranges.push(Range{start: split[0], end: split[1]});
        }

        else if ranges.contains(line.parse::<u128>().expect("Cannot convert to u128")) {
            fresh_count += 1;
        }
    }

    println!("Fresh count: {:?}", fresh_count);
}
