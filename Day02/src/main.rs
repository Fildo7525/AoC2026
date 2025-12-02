use std::fs;

#[derive(Debug)]
struct Range {
    start: i128,
    end: i128
}

fn get_ranges(input: &str) -> Vec<Range>
{
    let mut out: Vec<Range> = Vec::new();

    let ranges: Vec<i128> = input.split([',', '-']).map(|n| n.parse::<i128>().unwrap()).collect();
    for i in 0..ranges.len() {
        if i % 2 == 1 {
            continue;
        }

        out.push(Range{start: ranges[i], end: ranges[i+1]});
    }

    return out;
}


fn check_number(num: i128) -> i128
{
    let snum: Vec<u8> = num.to_string().bytes().collect();
    if snum.len() % 2 != 0 { return 0; }
    let f = &snum[0..snum.len()/2];
    let l = &snum[snum.len()/2..snum.len()];

    if f == l { return num; }
    return 0;
}

fn main() {
    let mut contents = fs::read_to_string("./src/puzzle2.txt").expect("File does not exist");
    contents.pop();
    let ranges: Vec<Range> = get_ranges(&contents);

    let mut palindromes = 0;

    for range in ranges {
        for n in range.start..range.end+1 {
            palindromes += check_number(n);
        }
    }

    println!("{:#?}", palindromes);
}
