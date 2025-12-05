use std::time::SystemTime;
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

    fn get_size(&self) -> u128 {
        return self.end - self.start + 1u128;
    }

    fn clone(&self) -> Range
    {
        return Range{ start: self.start.clone(), end: self.end.clone() };
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

    fn combine(&mut self) -> Ranges
    {
        self.sort_by(|a, b| a.start.cmp(&b.start) );

        let mut new_ranges = Ranges(Vec::new());
        new_ranges.push(self.0[0].clone());

        for r in &self.0 {
            if r.end <= new_ranges.last().unwrap().end { continue; }

            if r.start-1 <= new_ranges.last().unwrap().end {
                new_ranges.last_mut().unwrap().end = r.end;
            }
            else {
                new_ranges.push(r.clone());
            }
        }

        return new_ranges;
    }

    fn size(&self) -> u128
    {
        let mut total: u128 = 0;
        for r in &self.0 {
            total += r.get_size();
        }

        return total;
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

fn main()
{
    let mut contents = fs::read_to_string("./src/puzzle2.txt").expect("Cannot read the file");
    contents.pop();

    let lines: Vec<&str> = contents.split('\n').collect();
    let pt = 2;

    if pt == 1 {
        let mut ranges: Ranges = Ranges(Vec::new());
        let mut fresh_count = 0;
        let mut finished = false;

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

    else {
        let start = SystemTime::now();
        let mut ranges: Ranges = Ranges(Vec::new());

        for line in lines {
            if line.is_empty() { break; }

            let split: Vec<u128> = line.split("-").map(|sn| sn.parse().unwrap()).collect();
            let range = Range{start: split[0], end: split[1]};
            ranges.push(range);
        }

        let new_ranges = ranges.combine();

        let total: u128 = new_ranges.size();
        let duration = SystemTime::now().duration_since(start).unwrap();

        println!("Total: {} in {:?}", total, duration);
    }
}
