use std::fs;

fn get_num(input: &str) -> i32
{
    let number: i32;
    if input.as_bytes()[0] == b'L' {
        let n = input.get(1..input.len()).expect("Input parsing error");
        number = n.parse::<i32>().expect("Cannot convert to number") * -1;
    }
    else {
        let n = input.get(1..input.len()).expect("Input parsing error");
        number = n.parse::<i32>().expect("Cannot convert to number");
    }
    return number;
}

fn main() {
    let mut contents = fs::read_to_string("./src/puzzle2.txt").expect("File does not exist");
    contents.pop();
    let split = contents.split("\n");

    let mut count = 0;
    let mut tick: i32 = 50;
    let mut last = 50;

    for input in split {
        let number: i32 = get_num(input);

        let add = (number / 100).abs();
        count += add;
        let number = number % 100;

        tick += number;
        let add = (tick / 101).abs();
        if add != 0 {
            count += add;
        }

        tick %= 100;
        if tick == 0 {
            count += 1;
        }
        if last * tick < 0 {
            count += 1;
        }
        if tick < 0 { tick += 100 }
        last = tick;
    }

    println!("Generated 0'es: {:?}", count)

}
