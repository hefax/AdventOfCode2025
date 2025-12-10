use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::str::FromStr;
use std::fmt;
use std::i64;
use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;


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

struct Machine {
    leds:i64,
    buttons: Vec<i64>,
}

fn parse_stuff(input:String) -> Vec<Machine> {

    let pattern = format!(r"^\[([.#]*)\] (.*) \{{(.*)\}}");
    let pattern2 = format!(r"\(([\d,]*)\)*");

    let re = Regex::new(pattern.as_str()).unwrap();
    let re2 = Regex::new(pattern2.as_str()).unwrap();

    let mut machines:Vec<Machine> = vec![];

    for (_,rivi) in input.lines().enumerate() {
        let parts = re.captures(rivi).unwrap();

        let leds = &parts[1];
        let power = &parts[3];

        let mut buttons:Vec<String> = vec![];

        let caps = re2.captures_iter(&parts[2]);

        for i in caps {
            let s = i[1].to_string();
            buttons.push(s);
        }

        // println!("l: {:?} p: {:?} b: {:?}",leds,power,buttons);
        
        // parse the leds into binary
        let l:Vec<char> = leds.chars().collect();
        
        let mut s:String = String::from("");
        for i in l.iter() {
            let s1 = match i {
                '#' => "1",
                '.'=> "0",
                _ => panic!("nope"),
            };
            s = format!("{}{}",s,s1);
        }
        let led_len=s.len();

        let intval:i64 = isize::from_str_radix(s.as_str(), 2).unwrap() as i64;
        println!("{:?} -> {} -> {}",leds,s,intval);

        // parse the buttons into binary and calcurate ALL combinations 
        let mut button_vals:Vec<i64> = vec![];

        for i in buttons.iter() {

            let lines:Vec<i64> = i.as_str()
                    .split(",")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|s| FromStr::from_str(s).unwrap())
                    .collect::<Vec<i64>>();
            

            let mut s:String = String::from("");
            for i in 0..led_len as i64{
                if lines.contains(&i) {
                    s = format!("{}{}",s,1);
                }
                else {
                    s = format!("{}{}",s,0);
                }
            } 
            let buttonval:i64 = isize::from_str_radix(s.as_str(), 2).unwrap() as i64;
            println!("{:?} -> {} -> {}",lines,s,buttonval);

            button_vals.push(buttonval);
        }

        println!("l: {:?} {} p: {:?} b: {:?} ({:?})",s,intval,power,buttons,button_vals);

        // ignore the jolts for now.
        //

        let m = Machine{
            leds:intval,
            buttons:button_vals,
        };

        machines.push(m);
    }

    machines
}



fn first(filename:&str) {
    let input = get_file_data(filename);

    let machines = parse_stuff(input);

    let mut res:i64 = 0;

    for i in machines {
        let mut result:Vec<Vec<i64>> = vec![];
        let set = i.buttons.iter().powerset().collect::<Vec<_>>(); 
        // println!("{:?}",set);
        let mut smol:i64 = 10000;
        //
        for combo in set {
            if combo.len() == 0 {
                continue;
            }

            let mut val = 0;

            for num in &combo {
                val = val ^ *num;
            }

    //        println!("{:?} {val}",combo);

            if val == i.leds {
                let mut tmp:Vec<i64> = vec![];
                for num in &combo {
                    tmp.push(**num);
                }
                if smol > tmp.len() as i64 {
                    smol = tmp.len() as i64; 
                }
                result.push(tmp);

                // we have a combination that works for this machine. 
            }
        }

        res += smol;
         

        
        println!("{:?}",result);
    }

    println!("and we have {res}");

}




fn second(filename:&str) {
    // do the same as the fist one,
    // create the map.
    // start creating rectagles. 
    // check the rectangle edges if they are in the given area. 
    // discard rectangles that are not. 
    
    let input = get_file_data(filename);

}




fn main() {
    first("input.txt");
    second("test.txt");
}
