use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::str::FromStr;
use std::fmt;
use std::i64;
use std::collections::HashMap;
// use regex::Regex;

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

#[derive(Clone, Debug)]
struct Pos {
    x:i64,
    y:i64
}
impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x,self.y)
    }
}


fn get_red(input:String) -> (HashMap<String,Pos>, Vec<String>) {

    let mut reds:HashMap<String,Pos> = HashMap::new();
    let mut order:Vec<String> = vec![];

    for (_,rivi) in input.lines().enumerate() {
        let coords:Vec<&str> = rivi.split(",").collect();

        let x:i64  = match FromStr::from_str(coords[0].trim()) {
            Ok(num) => num,
            Err(_) => panic!("jotain kusi"),
        };
        let y:i64  = match FromStr::from_str(coords[1].trim()) {
            Ok(num) => num,
            Err(_) => panic!("jotain kusi"),
        };
        let item=Pos{x:x,y:y};
        let name = item.to_string();
        order.push(name.clone());
    
        reds.insert(name,item);
    }
    
    (reds,order)
}

enum Cell {
    Free,
    RedL, // red that turns left
    RedR, // right
    GreenBorderH,
    GreenBorderV,
    GreenInner,
}


fn get_map(input:&HashMap<String,Pos>,order:&Vec<String>) -> Vec<Vec<Cell>> {

    let mut map:Vec<Vec<Cell>> = vec![];

    let mut x_max:i64=0;
    let mut y_max:i64=0;
    let mut x_min:i64=100000;
    let mut y_min:i64=100000;

    for (_key,pos) in input.iter() {
        if x_max < pos.x {
            x_max = pos.x +1;
        }   
        if y_max < pos.y {
            y_max = pos.y +1;
        }   

        if x_min > pos.x {
            x_min = pos.x -1;
        }

        if y_min > pos.y {
            y_min = pos.y -1;
        }
    }

    for _y in y_min..=y_max {
        let mut tmp:Vec<Cell> = vec![];
        for _x in x_min..=x_max {
            tmp.push(Cell::Free);
        }
        map.push(tmp);
    }
    
    let mut d:char = 'X';
    let mut pd:char = 'X';
    let mut id:char = 'X';
    let mut first:Option<Pos> = None;
    let mut last:Option<Pos> = None;
    for name in order {

        let pos = input.get(name).unwrap();

        match first {
            None => {
                first = Some(pos.clone())
            },
            Some(_) => {},
        }
        let prev:Pos = match last{
            None => {
                last = Some(pos.clone());
                continue;
            },
            Some(val) => {
                val
            },
        };

        println!("{:?} -> {:?}",prev,pos);

        if prev.x == pos.x {
            // we travel on y coord. 
            let mut range = prev.y+1..pos.y;

            d = 'D';
            if prev.y > pos.y {
                range = pos.y+1..prev.y;
                d = 'U';
            }
            println!("We travel in X {:?}",range);

            for y in range {
                map[y as usize][pos.x as usize] = Cell::GreenBorderV;
            }
        }
        if prev.y == pos.y {
            // we travel on x coord. 
            let mut range = prev.x+1..pos.x;


            d = 'R';
            if prev.x > pos.x {
                d='L';
                range = pos.x+1..prev.x;
            }
            println!("We travel in Y {:?}",range);

            for x in range {
                map[pos.y as usize][x as usize] = Cell::GreenBorderH;
            }
        }

        if pd == 'X' {
        }
        else if 
            (pd == 'L' && d == 'D') ||
            (pd == 'R' && d == 'U') ||
            (pd == 'D' && d == 'R') ||
            (pd == 'U' && d == 'L') 
        {
            println!("{pd} to {d} so start was a Left turn");
            // map[pos.y as usize][pos.x as usize] = Cell::RedL;
            map[prev.y as usize][prev.x as usize] = Cell::RedL;
        }
        else if 
            (pd == 'L' && d == 'U') ||
            (pd == 'R' && d == 'D') ||
            (pd == 'D' && d == 'L') ||
            (pd == 'U' && d == 'R') 
        {
            println!("{pd} to {d} so start was a Right turn");
            //map[pos.y as usize][pos.x as usize] = Cell::RedR;
            map[prev.y as usize][prev.x as usize] = Cell::RedR;
        }
        else {
            println!("{d} {pd}");
             panic!("WTF?");
        }


        pd=d;
        if id == 'X' {
            id =d;
        }

        last = Some(pos.clone());
    }

    let initial = match first {
        None => {
            panic!("Ok wtf?");
        },
        Some(val) => {
            val
        },
    };
    let prev:Pos = match last{
        None => {
            panic!("Ok wtf?");
        },
        Some(val) => {
            val
        },
    };



    if prev.x == initial.x {
        // we travel on y coord. 
        let mut range = prev.y..initial.y;

        d = 'D';
        if prev.y > initial.y {
            range = initial.y..prev.y;
            d = 'U';
        }
        println!("C We travel in X {:?}",range);

        for y in range {
            map[y as usize][initial.x as usize] = Cell::GreenBorderV;
        }
    }
    if prev.y == initial.y {
        // we travel on x coord. 
        let mut range = prev.x..initial.x;

        d = 'R';
        if prev.x > initial.x {
            range = initial.x..prev.x;
            d = 'L';
        }
        println!("C We travel in Y {:?}",range);

        for x in range {
            map[initial.y as usize][x as usize] = Cell::GreenBorderH;
        }
    }

    if 
        (pd == 'L' && d == 'D') ||
        (pd == 'R' && d == 'U') ||
        (pd == 'D' && d == 'R') ||
        (pd == 'U' && d == 'L') 
    {
            println!("C {pd} to {d} so start was a Left turn at {},{}",prev.x,prev.y);
        map[prev.y as usize][prev.x as usize] = Cell::RedL;
    }
    else if 
        (pd == 'L' && d == 'U') ||
        (pd == 'R' && d == 'D') ||
        (pd == 'D' && d == 'L') ||
        (pd == 'U' && d == 'U') 
    {
            println!("C {pd} to {d} so start was a Right turn at {},{}",prev.x,prev.y);
        map[prev.y as usize][prev.x as usize] = Cell::RedR;
    }
    else {
       // panic!("WTF?");
    }
    if 
        (d == 'L' && id == 'D') ||
        (d == 'R' && id == 'U') ||
        (d == 'D' && id == 'R') ||
        (d == 'U' && id == 'L') 
    {
            println!("C {pd} to {d} so start was a Left turn at {},{}",initial.x,initial.y);
        map[initial.y as usize][initial.x as usize] = Cell::RedL;
    }
    else if 
        (d == 'L' && id == 'U') ||
        (d == 'R' && id == 'D') ||
        (d == 'D' && id == 'L') ||
        (d == 'U' && id == 'R') 
    {
            println!("C {pd} to {d} so start was a Right turn at {},{}",initial.x,initial.y);
        map[initial.y as usize][initial.x as usize] = Cell::RedR;
    }
    else {
            println!("{d} {id}");
        panic!("WTF?");
    }

    let mut count:i64=0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            match map[y][x] {
                Cell::GreenBorderV => {
                    if count >= 1 {
                        count = 0;
                    }
                    else {
                        count = 1;
                    }
                }, 
                Cell::RedL => {
                    count+=1;
                },
                Cell::Free => {
                    if count >= 1 {
                        map[y as usize][x as usize] = Cell::GreenInner;
                    }
                }
                _ => {
                    // don't care
                },
            }
        }
        count=0;
    }

    // fill the map ö
    // make map. 
    
    map
}

fn print_map(map:&Vec<Vec<Cell>>) {

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let c:char = match map[y][x] {
                Cell::Free => {'.'}, 
                Cell::RedL => {'L'}, 
                Cell::RedR => {'R'}, 
                Cell::GreenBorderH => {'X'}, 
                Cell::GreenBorderV => {'V'}, 
                Cell::GreenInner => {'+'}, 
            };

            print!("{}",c);
        }
        println!(" ");

    }
}


fn first(filename:&str) {
    let input = get_file_data(filename);

    let (reds,_order) = get_red(input);

    let keys = Vec::from_iter(reds.keys());


    let mut areas:HashMap<i64,(Pos,Pos)> = HashMap::new();
    
    for item in keys {
        for (_key,r2) in reds.iter() {
            let r1 = reds.get(item).unwrap();
            let area:i64 = (i64::abs(r1.x-r2.x)+1) * (i64::abs(r1.y-r2.y)+1);

            areas.insert(
                    area,
                    (r1.clone(),r2.clone())
                );
        }
    }

    let mut k2 = Vec::from_iter(areas.keys());
    k2.sort_by(|a,b| a.cmp(b));

    for i in k2 {
        println!("{}: {:?}",i,areas.get(&i).unwrap());
    }


}

fn check_if_in(map:&Vec<Vec<Cell>>,r1:&Pos,r2:&Pos) -> bool {

    let mut y_min = 0;
    let mut x_min = 0;
    let mut y_max = 0;
    let mut x_max = 0;


    // I assume all the reds are inside the map.
    //


    if r1.y < r2.y {
        y_min = r1.y;
        y_max = r2.y;
    }
    else {
        y_min = r2.y;
        y_max = r1.y;
    }

    if r1.x < r2.x {
        x_min = r1.x;
        x_max = r2.x;
    }
    else {
        x_min = r2.x;
        x_max = r1.x;
    }


    for y in y_min..=y_max {
        for x in x_min..=x_max {
            match map[y as usize][x as usize] {
                Cell::Free => {
                    return false;
                }, 
                _ => {
                    // ok.
                }
            };

        }

    }

    true

}


fn second(filename:&str) {
    // do the same as the fist one,
    // create the map.
    // start creating rectagles. 
    // check the rectangle edges if they are in the given area. 
    // discard rectangles that are not. 
    
    let input = get_file_data(filename);

    let (reds,order) = get_red(input);
    println!("reds ok");

    let map = get_map(&reds,&order);
    print_map(&map);

    let mut areas:HashMap<i64,(Pos,Pos)> = HashMap::new();
    
    let keys = Vec::from_iter(reds.keys());

    for item in keys {
        for (_key,r2) in reds.iter() {
            let r1 = reds.get(item).unwrap();

            if check_if_in(&map,r1,r2)  {
                let area:i64 = (i64::abs(r1.x-r2.x)+1) * (i64::abs(r1.y-r2.y)+1);
                areas.insert(
                    area,
                    (r1.clone(),r2.clone())
                );
               // println!("added: {:?},{:?}",r1,r2)
            }



        }
    }

    let mut k2 = Vec::from_iter(areas.keys());
    k2.sort_by(|a,b| a.cmp(b));

    for i in k2 {
        println!("{}: {:?}",i,areas.get(&i).unwrap());
    }


    let mut k2 = Vec::from_iter(areas.keys());
    k2.sort_by(|a,b| a.cmp(b));

    for i in k2 {
        println!("{}: {:?}",i,areas.get(&i).unwrap());
    }


}




fn main() {
    first("test.txt");
    second("input.txt");
}
