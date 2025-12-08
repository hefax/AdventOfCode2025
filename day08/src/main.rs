use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;
use std::i64;
// use std::f64;
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
#[derive(Clone, Debug, PartialEq, Eq)]
enum Location {
    Free,
    Circuit(i64),
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
    // removed the sqrt from this. 
    let dx =( j2.x - j1.x ) as i64;
    let dy =( j2.y - j1.y ) as i64;
    let dz =( j2.z - j1.z ) as i64;
    dx*dx + dy*dy + dz*dz
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


fn first(filename:&str,steps:i64) {
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

    let mut status:HashMap<String,Location> = HashMap::new();

    for (name,_item) in items.iter() {
        status.insert(name.clone(),Location::Free);
    }


    let mut circuits:HashMap<i64,Vec<String>> = HashMap::new();

    let mut c:i64 = 0;
    let mut cons:i64 = 0;
    let mut i:i64 =0;
    while cons < steps {
        let (a,b) = map.get(keys[i as usize]).unwrap();
        println!("{}: {:?} -> {:?}",keys[i as usize],a,b);

        let first = a.to_string();
        let second = b.to_string();


        if status[&first] == Location::Free && 
           status[&second] == Location::Free {
            // create a new circuit by adding both
            // Junctions to it.
            println!("   {} and {} created a new circuit {}",first,second,c);
            let mut tmp:Vec<String> = vec![];
            tmp.push(first.clone());
            tmp.push(second.clone());
            circuits.insert(c,tmp);
            status.insert(first,Location::Circuit(c));
            status.insert(second,Location::Circuit(c));
            cons+=1;
            c+=1;
        }
        else if status[&first] != Location::Free && 
           status[&second] == Location::Free {
            // so a is already in circuit.
            //
            let t:i64 =  match status[&first] {
                Location::Circuit(t) => t,
                _ => panic!("should not happen"),
            };
            println!("   {} is already in {}. Add {} to the same.",first,t,second);

            let mut tmp:Vec<String> = circuits.get(&t).unwrap().to_vec();

            tmp.push(second.clone());
            cons+=1;

            circuits.insert(t,tmp);
            status.insert(second, Location::Circuit(t));
        }
        else if status[&first] == Location::Free && 
           status[&second] != Location::Free {
            // so a is already in circuit.
            let t:i64 =  match status[&second] {
                Location::Circuit(t) => t,
                _ => panic!("should not happen"),
            };
            println!("   {} is already in {}. Add {} to the same.",second,t,first);

            let mut tmp:Vec<String> = circuits.get(&t).unwrap().to_vec();

            tmp.push(first.clone());
            cons+=1;

            circuits.insert(t,tmp);
            status.insert(first, Location::Circuit(t));
        }
        else {
            // both are in a circuit. // combine them.
            let to:i64 =  match status[&first] {
                Location::Circuit(t) => t,
                _ => panic!("should not happen"),
            };
            let from:i64 =  match status[&second] {
                Location::Circuit(t) => t,
                _ => panic!("should not happen"),
            };
            println!("   Connection {} ({}) and {} ({}) to {}",to,first,from,second,to);

                cons+=1;
            if to != from {
                println!("----");
                let tmp:Vec<String> = circuits.get(&from).unwrap().to_vec();
                let mut tmp2:Vec<String> = circuits.get(&to).unwrap().to_vec();

                for n in tmp {
                    status.insert(n.clone(),Location::Circuit(to));
                    tmp2.push(n.clone());
                }

                circuits.insert(to,tmp2);
                circuits.remove(&from);
            }

        }
        i+=1;
    }
    
    let mut sizes:Vec<i64> = vec![];

    println!("----");
     for (k,v) in circuits.iter() {
         println!("{}:{:?} {}",k,v,v.len());
         sizes.push(v.len() as i64);

     }

    println!("----");
     for (k,v) in status.iter() {
         println!("{}: {:?}",k,v);
     }

    sizes.sort_by(|a,b| b.cmp(a));

     // println!("{:?}",sizes);
    println!("Final: {}",sizes[0]*sizes[1]*sizes[2]);
}


fn second(filename:&str) {
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

    let mut status:HashMap<String,Location> = HashMap::new();

    for (name,_item) in items.iter() {
        status.insert(name.clone(),Location::Free);
    }


    let mut circuits:HashMap<i64,Vec<String>> = HashMap::new();

    let mut c:i64 = 0;
    let mut i:i64 =0;
    loop {
        let (a,b) = map.get(keys[i as usize]).unwrap();
        println!("{}: {:?} -> {:?}",keys[i as usize],a,b);

        let first = a.to_string();
        let second = b.to_string();


        if status[&first] == Location::Free && 
           status[&second] == Location::Free {
            // create a new circuit by adding both
            // Junctions to it.
            println!("   {} and {} created a new circuit {}",first,second,c);
            let mut tmp:Vec<String> = vec![];
            tmp.push(first.clone());
            tmp.push(second.clone());
            circuits.insert(c,tmp);
            status.insert(first,Location::Circuit(c));
            status.insert(second,Location::Circuit(c));
            c+=1;
        }
        else if status[&first] != Location::Free && 
           status[&second] == Location::Free {
            // so a is already in circuit.
            //
            let t:i64 =  match status[&first] {
                Location::Circuit(t) => t,
                _ => panic!("should not happen"),
            };
            println!("   {} is already in {}. Add {} to the same.",first,t,second);

            let mut tmp:Vec<String> = circuits.get(&t).unwrap().to_vec();

            tmp.push(second.clone());

            circuits.insert(t,tmp);
            status.insert(second, Location::Circuit(t));
        }
        else if status[&first] == Location::Free && 
           status[&second] != Location::Free {
            // so a is already in circuit.
            let t:i64 =  match status[&second] {
                Location::Circuit(t) => t,
                _ => panic!("should not happen"),
            };
            println!("   {} is already in {}. Add {} to the same.",second,t,first);

            let mut tmp:Vec<String> = circuits.get(&t).unwrap().to_vec();

            tmp.push(first.clone());

            circuits.insert(t,tmp);
            status.insert(first, Location::Circuit(t));
        }
        else {
            // both are in a circuit. // combine them.
            let to:i64 =  match status[&first] {
                Location::Circuit(t) => t,
                _ => panic!("should not happen"),
            };
            let from:i64 =  match status[&second] {
                Location::Circuit(t) => t,
                _ => panic!("should not happen"),
            };
            println!("   Connection {} ({}) and {} ({}) to {}",to,first,from,second,to);

            if to != from {
                println!("----");
                let tmp:Vec<String> = circuits.get(&from).unwrap().to_vec();
                let mut tmp2:Vec<String> = circuits.get(&to).unwrap().to_vec();

                for n in tmp {
                    status.insert(n.clone(),Location::Circuit(to));

                    //if tmp2.contains(&n) == false {
                        tmp2.push(n.clone());
                   // }
                }

                circuits.insert(to,tmp2);
                circuits.remove(&from);
            }

        }
        
        let mut cont=false;
        for (_name,stat) in status.iter() {

            if *stat == Location::Free {
                cont=true;

                break;
            }
            
        }
        if cont {
            i+=1;
        }
        else {
            break;
        }
    }


    println!("Last coords {:?}", map.get(keys[i as usize]).unwrap());

    let (a,b) = map.get(keys[i as usize]).unwrap();

    let res = a.x*b.x;

    println!("Result: {}",res);

}

fn main() {
    first("test.txt",10);
    second("test.txt");
}
