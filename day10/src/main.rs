use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::str::FromStr;
use std::fmt;
use std::i64;
use std::collections::HashMap;
use regex::Regex;
use itertools::Itertools;
use std::thread;


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
struct Machine {
    leds:i64,
    buttons: Vec<Button>,
    jolts:Vec<i64>,
}

#[derive(Clone, Debug)]
struct Button {
    code:i64,
    text:String,
    bin:String,
    set:Vec<i64>,
}

impl fmt::Display for Button {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.text)
    }
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
        let mut button_list:Vec<Button> = vec![];

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

            let but = Button {
                code:buttonval,
                text:i.clone(),
                bin:bs,
                set:lines.clone(),
            };

            button_list.push(but);
        }


        // ignore the jolts for now.
        //
        let jolts:Vec<i64> = power.split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| FromStr::from_str(s).unwrap())
            .collect::<Vec<i64>>();
            
        println!("l: {:?} {} p: {:?} b: {:?} ",s,intval,jolts,button_list);

        let m = Machine{
            leds:intval,
            buttons:button_list,
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
        let set = i.buttons.iter()
            .map(|x| x.code)
            .powerset()
            .collect::<Vec<_>>(); 
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
                    tmp.push(*num);
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

fn get_buttons(jolts:&Vec<i64>,buttons:&Vec<Button>,mut depth:i64,mut book:Vec<String>) -> Option<Vec<Vec<i64>>> {
    let size:i64 =jolts.len() as i64;


    let mut res:Vec<Vec<i64>> = vec![];

    depth-=1;

    if depth < 0 {
        return None;
    }

    let mut filtered:HashMap<i64,Vec<i64>> = HashMap::new();
    
    for (index,button) in buttons.iter().enumerate() {
        let mut tmp = jolts.clone();

        for i in &button.set {
            tmp[*i as usize] -=1;
        }


        let mut done = true;
        let mut skip = false;
        let mut sanity = false;

        for i in 0..size {
            if tmp[i as usize] != jolts[i as usize] {
                sanity = true;
            }


            if tmp[i as usize] < 0 {
                skip=true;
                break;
            }

            if tmp[i as usize] > 0 {
                done = false;
            }
        }

        // we do not want this button as it will cause us to 
        // go to negatives
        if skip {
            continue;
        }

        if done {
            // we found a path to 0. every line was zero
            // we need to return Vec<Vec<Button>>
            // none of the other buttons will give this result so we can stop.
            let mut r:Vec<i64>= vec![];
            r.push(index as i64);
            res.push(r);
            return Some(res);
        }

        if sanity == false {
            panic!("we did nothing");
        }

        filtered.insert(index as i64,tmp);
    }

    // println!("{:?} -> {:?}",jolts,filtered);
    if filtered.len() == 0 {
        // dead end 
        
        return None;
    }



    for (index, jolts) in filtered.iter() {

        let check:String = jolts.iter().map(|x| x.to_string()).collect::<String>();

        if book.contains(&check) {
            continue;
        }
        
        book.push(check);

        match get_buttons(&jolts,&buttons,depth,book.clone()) {
            Some(path) => {
                // we have at least one path that were ok with 
                // this button.
                for p in path {
                    let mut r = p.clone();
                    
                    r.push(*index as i64);

                    res.push(r);
                }
                // free up some memory as we do not need to keep
                // tabs on every single path. only the shortes. 
                let mut min:i64 = -1;
                let mut min_index = 0;
                for (ii,pp) in res.iter().enumerate() {
                    if min < 0 {
                        min = pp.len() as i64;
                        min_index = ii;
                    }               
                    else if min > pp.len() as i64 {
                        min = pp.len() as i64;
                        min_index = ii;
                    }
                }
                let shortest = res[min_index].clone();
                res.clear();
                res.push(shortest);
            }
            None => {
                // this button press was a failure. 
            }
        }
    } 

    if res.len() > 0 {
        return Some(res);
    }
    None
}


fn second(filename:&str) {
    
    let input = get_file_data(filename);
    let machines = parse_stuff(input);

    let mut res:i64 = 0;
    println!("We go.");

    
    
    for chunck in machines.chunks(16) {
        let mut threats = vec![];    
        for i in chunck {
            

            //let mut result:Vec<Vec<Button>> = vec![];
            let mut book:Vec<String> = vec![];

            let tmp = i.clone();
            
            let handle = thread::spawn(move || {
                let result = get_buttons(&tmp.jolts,&tmp.buttons,1000,book);
                let mut min:i64=100000;

                match result {
                    None => {println!("we failed")},
                    Some(result) => {
                        let mut min_i:i64=0;
                        for (i,data) in result.iter().enumerate() {
                            if min > data.len() as i64{
                                min_i = i as i64;
                                min = data.len() as i64;
                            }
                        }
                        println!("shortest: {}: {:?}",result[min_i as usize].len(),result[min_i as usize]);

                        
                    }
                }

                min
            });

            threats.push(handle);
        }
        for i in threats {
            let d = i.join().unwrap();
            res +=d;
        }

    }

    println!("and we have {res}");


}




fn main() {
    first("test.txt");
    second("test.txt");
}
