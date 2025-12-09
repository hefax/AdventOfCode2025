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


fn get_red(input:String) -> HashMap<String,Pos> {

    let mut reds:HashMap<String,Pos> = HashMap::new();

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
        reds.insert(item.to_string(),item);
    }
    
    reds
}

fn get_map(input:&HashMap<String,Pos>) -> Vec<Vec<char>> {

    let mut map:Vec<Vec<char>> = vec![];

    for (_,pos) in input.iter().enumerate() {
        

    
        let item=Pos{x:x,y:y};
        reds.insert(item.to_string(),item);
    }
    // make map. 
    
    reds
}

fn first(filename:&str) {
    let input = get_file_data(filename);

    let reds = get_red(input);

    let mut keys = Vec::from_iter(reds.keys());


    let mut areas:HashMap<i64,(Pos,Pos)> = HashMap::new();
    
    for item in keys {
        for (key,r2) in reds.iter() {
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

fn second(filename:&str) {
    // do the same as the fist one,
    // create the map.
    // start creating rectagles. 
    // check the rectangle edges if they are in the given area. 
    // discard rectangles that are not. 
    

}




fn main() {
    first("input.txt");
//    println!("----------");
  //  second("input.txt");
}
