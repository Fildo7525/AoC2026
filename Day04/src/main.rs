use std::fs;

struct Point {
    x: usize,
    y: usize,
}

fn count_ats(p: &Point, field: &Vec<&str>) -> usize
{
    let mut cnt: usize = 0;
    let h = field.len();
    let w = field[0].len();

    for dy in -1..=1 as i32 {

        let ny = p.y as i32 + dy;
        if ny < 0 || ny >= h as i32 { continue; }
        let line: Vec<char> = field[ny as usize].chars().collect();


        for dx in -1..=1 as i32 {
            if dy == 0 && dx == 0 { continue; }

            let nx = p.x as i32 + dx;

            if nx < 0 || nx >= w as i32 { continue; }


            if line[nx as usize] == '@' {
                // println!("Identifying: ({},{})", ny, nx);
                cnt += 1;
            }
        }
    }

    return cnt;
}

fn main() {
    let mut contents = fs::read_to_string("./src/puzzle2.txt").expect("Cannot read the file");
    contents.pop();

    let lines: Vec<&str> = contents.split("\n").collect();
    // println!("Lines\n{:?}",lines);

    let mut count = 0;


    for y in 0..lines.len() {
        let line: Vec<char> = lines[y].chars().collect();

        for x in 0..lines[y].len() {
            if line[x] != '@' { continue; }

            let c = count_ats(&Point{x,y}, &lines);
            if c <= 3 {
                // println!("Collectable ({:2},{:2})\n", y, x);
                count += 1;
            }
        }
    }

    println!("{:?}", count);
}
