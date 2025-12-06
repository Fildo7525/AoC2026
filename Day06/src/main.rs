use std::{char::from_digit, fs};

struct Column {
    numbers: Vec<i128>,
    operation: String,
}

impl Column
{
    fn execute(&self) -> i128
    {
        if self.operation == "*" {
            let mut out = 1;
            for n in &self.numbers {
                out *= n;
            }
            return out;
        }
        else if self.operation == "+" {
            let mut out = 0;
            for n in &self.numbers {
                out += n;
            }
            return out;
        }

        return 0;
    }

    fn execute_pt2(&self)
    {
        let mut snums: Vec<String> = self.numbers.iter().map(|n| n.to_string()).collect();
        let longest = snums.iter().map(|s| s.len()).max().unwrap_or(0);

        for sn in &mut snums {
            if sn.len() == longest { continue; }

            let spacing = " ".repeat(longest - sn.len());
            let padded = format!("{}{}", spacing, sn);
            *sn = padded;
        }
    }
}

fn main() {
    let mut contents = fs::read_to_string("./src/puzzle2.txt").expect("Cannot read the file");
    contents.pop();

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut columns: Vec<Column> = Vec::new();


    for (idx, line) in lines.iter().enumerate() {
        let l = line.to_string();
        if idx < lines.len()-1 {
            let nums: Vec<&str> = l.split(' ').collect();

            let mut i = 0;
            for sn in nums {
                if sn.is_empty() { continue; }

                let n: i128 = sn.parse().unwrap();

                if idx == 0 {
                    columns.push(Column{ numbers: vec!{n}, operation: String::from("")});
                }
                else {
                    columns[i].numbers.push(n);
                    i += 1;
                }

            }
        }
        else {
            let mut i = 0;
            for op in l.split(' ') {
                if op.is_empty() { continue; }
                println!("Setting op {} for col {}", op, i);
                columns[i].operation = String::from(op);
                i += 1;
            }
        }
    }

    let mut result: i128 = 0;
    for op in &columns {
        let r = op.execute();
        println!("sub result: {}", r);
        result += r;
    }

    println!("Result: {}", result);
}
