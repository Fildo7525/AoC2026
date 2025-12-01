use std::fs;

fn main() {
    let mut contents = fs::read_to_string("./src/puzzle2.txt").expect("File does not exist");
    contents.pop();
    let split = contents.split("\n");
    // println!("{:#?}", split);

    let mut count = 0;

    let mut tick: i32 = 50;

    let mut number: i32;
    for input in split {
        if input.as_bytes()[0] == b'L' {
            let n = input.get(1..input.len()).expect("Input parsing error");
            number = n.parse::<i32>().expect("Cannot convert to number") * -1;
        }
        else {
            let n = input.get(1..input.len()).expect("Input parsing error");
            number = n.parse::<i32>().expect("Cannot convert to number");
        }

        tick += number;
        tick %= 100;
        if tick == 0 {
            count += 1;
        }
    }

    println!("Generated 0'es: {:?}", count)

}
