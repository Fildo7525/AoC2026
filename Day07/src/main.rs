use std::fs;

fn main() {
    let mut contents = fs::read_to_string("./src/puzzle2.txt").expect("Cannot read the file");
    contents.pop();

    let lines: Vec<&str> = contents.split('\n').collect();
    println!("Number of lines: {}", lines.len());
}
