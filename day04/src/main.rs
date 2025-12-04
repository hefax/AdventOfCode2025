use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn get_file_data(filename:&str) -> String {
    let mut input = String::from("");

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            match line {
                Ok(rivi) => {
                    input = format!("{}{}\n",input,rivi);
                }
                Err(_) => {
                    panic!("reading failed!");
                }
            }
        }
    }
    input
}

fn check(map:&Vec<Vec<char>>,x:i32,y:i32) -> bool {
    
    if map[y as usize][x as usize] == '.' {
        return false;
    }

    let mut count:i32 =0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dy == 0 && dx == 0 {
                continue;
            }

            if y+dy >= 0 && y+dy < map.len() as i32{
                if x+dx >= 0 &&  x+dx < map[(y+dy) as usize].len() as i32{
                    if map[(y+dy) as usize][(x+dx) as usize] == '@' {
                        count +=1;
                    }
                }
            }
        }
    }

    if count < 4 {
        return true;
    }

    false
}

fn first(filename:&str) {
    let input = get_file_data(filename);

    //let mut zero:Vec<i64>=vec![];
    let mut map:Vec<Vec<char>> = vec![];

    for (_,rivi) in input.lines().enumerate() {
        let row_r: Vec<char> = rivi
            .chars()
            .collect();

        map.push(row_r);

    }
    
    let mut count:i32 = 0;
    for y in 0..map.len() {
        for x in 0..map[y as usize].len() {
            let ok = check(&map,x as i32,y as i32);

            if ok {
                count +=1;
            }
        }
    }


    println!("{}",count);

}



fn second(filename:&str) {
    let input = get_file_data(filename);

    //let mut zero:Vec<i64>=vec![];
    let mut map:Vec<Vec<char>> = vec![];

    for (_,rivi) in input.lines().enumerate() {
        let row_r: Vec<char> = rivi
            .chars()
            .collect();

        map.push(row_r);

    }
    
    let mut count:i32 = 0;
    loop {
        let mut this_round= 0;

        let mut remove:Vec<(i32,i32)> = vec![];

        for y in 0..map.len() {
            for x in 0..map[y as usize].len() {
                let ok = check(&map,x as i32,y as i32);

                if ok {
                    this_round +=1;
                    remove.push((x as i32,y as i32));
                }
            }
        }

        for i in remove {
            map[i.1 as usize][i.0 as usize]='.';
        }

        count += this_round;

        if this_round == 0 {
            break;
        }
    }


    println!("{}",count);

}

fn main() {
    first("input.txt");

    second("input.txt");
}
