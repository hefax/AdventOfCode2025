use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::str::FromStr;

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


fn first(filename:&str) {
    let input = get_file_data(filename);

    //let mut zero:Vec<i64>=vec![];
    let mut zero:i32=0;
    let mut current:i32=50;

    for (_,rivi) in input.lines().enumerate() {
        
        let dir_code = rivi.chars().nth(0).unwrap();
        let mut step = String::new();

        let dir = match dir_code {
            'L' => {
                -1
            },
            'R' => {
                1
            },
            _ => panic!("ja mitäs vittua taas!"),
        };

        if let Some(part) = rivi.get(1..) {
            step = part.to_string()
        }

        let step_num:i32  = match FromStr::from_str(step.as_str()) {
            Ok(num) => num,
            Err(_) => panic!("jotain kusi"),
        };
        
        current += dir*step_num;

        while current < 0 {
            current += 100;
        }

        while current > 99 {
            current -=100;
        }


        if current == 0 {
            zero +=1;
        }
        println!("Dial rotated {rivi} to point at {current}");


    }
    println!("Final: {zero}");

}

fn second(filename:&str) {
    let input = get_file_data(filename);

    //let mut zero:Vec<i64>=vec![];
    let mut zero:i32=0;
    let mut current:i32=50;

    //println!("What?: {input}");
    for (_,rivi) in input.lines().enumerate() {
        
        let dir_code = rivi.chars().nth(0).unwrap();
        let mut step = String::new();

        let dir = match dir_code {
            'L' => {
                -1
            },
            'R' => {
                1
            },
            _ => panic!("ja mitäs vittua taas!"),
        };

        if let Some(part) = rivi.get(1..) {
            step = part.to_string()
        }

        let mut step_num:i32  = match FromStr::from_str(step.as_str()) {
            Ok(num) => num,
            Err(_) => panic!("jotain kusi"),
        };
        

        let mut full=0;

        let stop = String::from("");

        // ok me ollaan 0, tai me ollaan positiivinen. 
        // jos me liikutaan vasemmalle kun ollaan nollassa me ei voida 
        let start = format!("{current} => ");


        // we have numbers bigger than 100. So minimize the fucker. 
        while step_num > 99 {
            step_num -=100;
            full+=1;
        }

        // the fucking thing.. if you start from zero and move 
        // smaller the logic shits itself. lets move the needle 
        // backwards by one step and we get the right answer.
        if current == 0 && dir < 0 {
            full-=1;
        }

        current += dir*step_num;

        // if we landed on zero 
        if current == 0 {
            full+=1;
        }

        // negative values and handled. by this. Note that starting from zero 
        // is not the same as passing zero. See the if at row 136
        while current < 0 { 
            current += 100;
            full+=1;
        }

        // positive loops. 
        while current > 99 {
            current -=100;
            full+=1;
        }

        zero+=full;  

        if full > 0 {
            println!("{start} rotated {rivi} to point at {current}, 0 passed {full} times.{stop}");
        }
        else {
            println!("{start} rotated {rivi} to point at {current}{stop}");

        }


    }
    println!("Final: {zero}");

}

fn main() {
    
    println!("Hello, world!");
    first("input.txt");
    second("input.txt");
    first("test.txt");
    second("test.txt");
}
