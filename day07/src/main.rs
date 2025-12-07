use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::collections::HashMap;
// use std::str::FromStr;
// use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

//atype Beam = (i64,i64);

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

fn tachyon_step_down(map:&Vec<Vec<char>>,x_list:Vec<i64>,y:i64) -> (Vec<i64>,i64) {
    
    let mut next:Vec<i64> = vec![];
    let mut splits:i64 = 0;

    print!("row {}:",y);
    for x in x_list {
        match map[y as usize][x as usize] {
            'S' => {
                // start
                print!(" {}  ",x);
                if next.contains(&(x)) == false {
                    next.push(x);
                }
            },
            '.' => {
                print!(" {}  ",x);
                // just air so the beam goes down.
                if next.contains(&(x)) == false {
                    next.push(x);
                }
            },
            '^' => {
                // splitter
                print!(" {}s ",x);
                splits +=1;
                //
                if x > 0 {
                    if next.contains(&(x-1)) == false {
                       next.push(x-1);
                    }
                }

                if x < map[y as usize].len() as i64+1 {
                    if next.contains(&(x+1)) == false {
                       next.push(x+1);
                    }
                }
                //
            },
            _ => panic!("why?"),
        }
    }

    println!(" ({})",splits);
    (next,splits)
}


fn tachyon_step_down_q(map:&Vec<Vec<char>>,x_list:HashMap<i64,i64>,y:i64) -> (HashMap<i64,i64>,i64) {
    
    let mut next:HashMap<i64,i64> = HashMap::new();

    let mut splits:i64 = 0;

    for (x,b) in x_list {
        match map[y as usize][x as usize] {
            'S' => {
                *next.entry(x).or_insert(0) +=b;
            },
            '.' => {
                *next.entry(x).or_insert(0) +=b;
            },
            '^' => {
                if x > 0 {
                    match next.get_mut(&(x-1)) {
                        Some(i) => {
                            *i += b;
                        },
                        None => {
                            next.insert(x-1, b);
                        }
                    }
                }

                if x < map[y as usize].len() as i64+1 {
                    match next.get_mut(&(x+1)) {
                        Some(i) => {
                            *i += b;
                        },
                        None => {
                            next.insert(x+1, b);
                        }
                    }
                }
            },
            _ => panic!("why?"),
        }
    }

    let mut sum:i64 = 0;
    for (k,v) in next.iter() {
        
        sum+=v;

    }
    println!(" ({:?}) == {}",next,sum);
    (next,splits)
}

fn first(filename:&str) {
    let input = get_file_data(filename);

    let mut splits:i64 = 0;
    let mut map:Vec<Vec<char>> = vec![];

    let mut beams:Vec<i64> = vec![];

    for (_,rivi) in input.lines().enumerate() {
        let clist:Vec<char> = rivi.chars().collect();
        map.push(clist);
    }

    for x in 0..map[0].len() {
        match map[0][x] {
            'S' => {
                beams.push(x as i64);
            },
            _ => { 
                // don't care at this moment.
            },
        }
    }
    
    for y in 0..map.len() {
        let (next_beams,tmp) = tachyon_step_down(&map,beams,y as i64);
 
        beams = next_beams;        

        splits += tmp;
    }
    
    println!("we split {splits} times.");
}


fn second(filename:&str) {
    let input = get_file_data(filename);

    let mut splits:i64 = 0;
    let mut map:Vec<Vec<char>> = vec![];

    let mut beams:HashMap<i64,i64>= HashMap::new();

    for (_,rivi) in input.lines().enumerate() {
        let clist:Vec<char> = rivi.chars().collect();
        map.push(clist);
    }

    for x in 0..map[0].len() {
        match map[0][x] {
            'S' => {
                beams.insert((x as i64),1);
            },
            _ => { 
                // don't care at this moment.
            },
        }
    }
    
    for y in 0..map.len() {
        let (next_beams,tmp) = tachyon_step_down_q(&map,beams,y as i64);
 
        beams = next_beams;        

        splits += tmp;
    }
    
    println!("we split {splits} times.");
}




fn main() {
    first("test.txt");
    second("input.txt");
}
