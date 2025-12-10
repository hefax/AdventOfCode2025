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
    buttons_bin:Vec<String>,
    jolts:Vec<i64>,
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
        let mut button_bins:Vec<String> = vec![];

        for i in buttons.iter() {

            let lines:Vec<i64> = i.as_str()
                    .split(",")
                    .collect::<Vec<&str>>()
                    .iter()
                    .map(|s| FromStr::from_str(s).unwrap())
                    .collect::<Vec<i64>>();
            

            let mut bs:String = String::from("");
            for i in 0..led_len as i64{
                if lines.contains(&i) {
                    bs = format!("{}{}",bs,1);
                }
                else {
                    bs = format!("{}{}",bs,0);
                }
            } 
            let buttonval:i64 = isize::from_str_radix(bs.as_str(), 2).unwrap() as i64;
            println!("{:?} -> {} -> {}",lines,bs,buttonval);

            button_vals.push(buttonval);
            button_bins.push(bs);
        }


        // ignore the jolts for now.
        //
        let jolts:Vec<i64> = power.split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| FromStr::from_str(s).unwrap())
            .collect::<Vec<i64>>();
            
        println!("l: {:?} {} p: {:?} b: {:?} ({:?})",s,intval,jolts,buttons,button_vals);

        let m = Machine{
            leds:intval,
            buttons:button_vals,
            buttons_bin:button_bins,
            jolts:jolts,
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
     //    println!("XX {:?}",set);
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

            println!("{:?} {val}",combo);

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


fn get_button(jolts:&Vec<i64>,buttons:&Vec<String>) {
    let size:i64 =jolts.len() as i64;

    // select the most important lines. (== highest jolt value.)
    let mut map:HashMap<i64,i64> = HashMap::new(); 
    // val -> index


    for (i,val) in jolts {
        map.insert(val,i);
    }

    let mut keys = Vec::from_iter(map.keys());
    keys.sort_by(|a,b| a.cmp(b));



    for i in keys {
        let index = map.get(&i).unwrap();

        if 
    }


    // select the button that addresses the needs in order (and does 
    // NOT put anything in negatives)



    // return button. 
}


fn second(filename:&str) {
    
    let input = get_file_data(filename);
    let machines = parse_stuff(input);

    let mut res:i64 = 0;

    for i in machines {
        let mut result:Vec<Vec<i64>> = vec![];

        
        // strategy:
        // select the buttons to closely match the option.
        // push button. Repeat. 
        //




        
        println!("{:?}",result);
    }

    println!("and we have {res}");


}




fn main() {
    first("test.txt");
    second("test.txt");
}
