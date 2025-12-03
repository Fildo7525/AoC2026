use std::fs;

fn main() {
    let mut contents = fs::read_to_string("./src/puzzle2.txt").expect("File does not exist");
    contents.pop();
    let mut sum: i128 = 0;

    for line in contents.split("\n") {

        let b: Vec<char> = line.chars().collect();
        let max = b[..b.len()-1].iter().max_by_key(|&k| k).expect("No max found");
        let max_pos = b.iter().position(|k| k == max).expect("Max out of range");

        let sub: Vec<&char> = b[max_pos+1..b.len()].iter().collect();
        println!("sub: {:?}", sub);
        let second_max = b[max_pos+1..b.len()].iter().max_by_key(|&k| k).expect("No other numbers after max");

        let mut num: i32 = (*max).to_digit(10).expect("No max num") as i32;
        num *= 10;
        num = num + (*second_max).to_digit(10).expect("No sencond max") as i32;

        sum += num as i128;
    }

    println!("Joltage: {:}", sum);
}
