use std::fs;

fn main() {
    let mut contents = fs::read_to_string("./src/puzzle2.txt").expect("File does not exist");
    contents.pop();
    let mut sum: i128 = 0;

    for line in contents.split("\n") {
        let b: Vec<char> = line.chars().collect();

        let mut sorted: Vec<char> = b[0..b.len()-12].to_vec();
        sorted.sort_by(|a, b| b.cmp(a));

        // println!("Origin: {:?}", b);
        // println!("Sorted: {:?}", sorted);

        let max = &sorted[0];
        let mut last_pos = b.iter().position(|k| *k == *max).expect("Max out of range") + 1;
        let mut num: i128 = (*max).to_digit(10).expect("No max num") as i128;

        for i in (0..11).rev() {
            let sub: Vec<&char> = b[last_pos..b.len()-i].iter().collect();
            // println!("Sub({:?},{:?}): {:?}", last_pos, b.len()-i, sub);
            let second_max = *sub.iter().max_by_key(|k| ***k).expect("No other numbers after max");
            // println!("Second max: {:}", second_max);
            last_pos += sub.iter().position(|k| **k == *second_max).expect("Max out of range") + 1;
            // println!("Last pos: {:}", last_pos);

            num *= 10;
            num = num + (*second_max).to_digit(10).expect("No sencond max") as i128;

        }

        // println!("Num: {:}", num);
        sum += num as i128;


        // let sub: Vec<&char> = b[max_pos+1..b.len()].iter().collect();
        // let second_max = b[max_pos+1..b.len()].iter().max_by_key(|&k| k).expect("No other numbers after max");

    }

    println!("Joltage: {:}", sum);
}
