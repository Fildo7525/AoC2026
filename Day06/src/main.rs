use std::fs;


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
}

#[derive(Debug)]
struct SColumn {
    numbers: Vec<i128>,
    operation: String,
    block_size: usize,
    start_idx: usize,
}

impl SColumn
{
    fn execute(&self) -> i128
    {
        if self.operation == "*" {
            let mut out = 1;
            for n in &self.numbers {
                out *= n;
            }
            println!("Sub result for *: {}", out);
            return out;
        }
        else if self.operation == "+" {
            let mut out = 0;
            for n in &self.numbers {
                out += n;
            }
            println!("Sub result for +: {}", out);
            return out;
        }

        return 0;
    }
}

fn pt1()
{
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

fn pt2()
{
    let mut contents = fs::read_to_string("./src/puzzle2.txt").expect("Cannot read the file");
    contents.pop();

    let lines: Vec<&str> = contents.split('\n').collect();
    let mut columns: Vec<SColumn> = Vec::new();

    let operations = lines.last().unwrap().chars().collect::<Vec<char>>();
    let mut last_op_idx = 0;
    loop {
        let op = operations[last_op_idx..].iter().find(|c| c != &&' ');
        match op {
            Some(c) => {
                let next_op_idx = operations[last_op_idx+1..].iter().position(|x| x != &' ').unwrap_or(operations.len() - last_op_idx);
                println!("Found op {} at idx {}, next_op_idx: {}", c, last_op_idx,  next_op_idx);
                let nums: Vec<i128> = vec![0; next_op_idx];
                println!("Vecor: {:?}", nums);

                columns.push(SColumn{ numbers: nums, operation: c.to_string(), block_size: next_op_idx, start_idx: last_op_idx});
                last_op_idx += next_op_idx + 1;
                if last_op_idx >= operations.len() {
                    break;
                }
            },
            None => break,
        }
    }

    for line in lines.iter().take(lines.len()-1)
    {
        let l = line.chars().collect::<Vec<char>>();
        for op in &mut columns {
            let nums: Vec<u8> = l[op.start_idx..op.start_idx+op.block_size].iter().map(|c| c.to_digit(10).unwrap_or(255) as u8 ).collect();
            // println!("Nums for col {}: {:?}", op.operation, nums);

            for (idx, n) in nums.iter().enumerate() {
                if *n == 255 { continue; }
                op.numbers[idx] = op.numbers[idx] * 10 + (*n as i128);
            }
        }
    }

    let mut result: i128 = 0;
    for op in &columns {
        let r = op.execute();
        result += r;
    }

    println!("Result: {}", result);
}


fn main()
{
    // pt1();
    pt2();
}
