use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;
use std::i64;
use std::f64;
// use regex::Regex;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> 
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone, Debug)]
struct Junction {
    x: i64,
    y: i64,
    z: i64,
}

impl fmt::Display for Junction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{},{}", self.x,self.y,self.z)
    }
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

fn get_distance(j1:&Junction,j2:&Junction) -> i64 {
    let dx =( j2.x - j1.x ) as i64;
    let dy =( j2.y - j1.y ) as i64;
    let dz =( j2.z - j1.z ) as i64;
    (dx*dx + dy*dy + dz*dz)
}

fn map_distance(j:&Junction,list:&HashMap<String,Junction>) -> HashMap<String,i64> {
    
    let mut result: HashMap<String,i64> = HashMap::new();

    let orig = j.to_string();
    for (name,item) in list.iter() {
        if *name == orig {
            continue;
        }

        let d = get_distance(&j,&item);

        result.insert(item.to_string(),d);
    }
    
    result
}


fn first(filename:&str) {
    let input = get_file_data(filename);

    let mut items: HashMap<String,Junction> = HashMap::new();

    for (_,rivi) in input.lines().enumerate() {
        let coord_r:Vec<i64> = rivi.split(",")
            .map(|x| i64::from_str(x.trim()).expect("Failed to parse string."))
            .collect::<Vec<_>>();

        let coord = Junction {
                x: coord_r[0],
                y: coord_r[1],
                z: coord_r[2],
        };
        
        items.insert(coord.to_string(),coord);
    }

    // let mut result: HashMap<String,f64> = HashMap::new();
    let mut map: HashMap<i64,(Junction,Junction)> = HashMap::new();
    for (name,item) in items.iter() {

        let set = map_distance(&item,&items);

        for (node,distance) in set.iter() {
            map.insert(*distance,
                (
                    items.get(name).unwrap().clone(),
                    items.get(node).unwrap().clone()
                )
            );
        }
    }

    let mut keys = Vec::from_iter(map.keys());
    keys.sort_by(|a,b| a.cmp(b));

    for i in 0..10 {
        println!("{} {:?}",keys[i],map.get(keys[i]).unwrap());

    }
    
    // ok idea oli. distance -> pairs info.
    // go though distances and connect nodes. 
    // group nodes and handle mapping that way.


    // println!("{:?}",map);
}


fn main() {
    first("test.txt");
}
