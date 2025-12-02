use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::str::FromStr;
use regex::Regex;

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

fn check_code(id:i64) -> bool {
    
    let foo = format!("{id}");
    
    let len = foo.chars().count();

    if len % 2 != 0 {
        return false;
    }

    for i in 0..len/2 {
        let j = (len/2)+i;

        if foo.chars().nth(i).unwrap() != foo.chars().nth(j).unwrap() {
            return false;
        }
    }

    true 
}

fn check_code2(id:i64) -> bool {
    let foo = format!("{id}");
    
    let len = foo.chars().count();

    for i in 1..=len/2 {
        let s = &foo[..i];

        let pattern = format!(r"^({})+$",s);
        let re = Regex::new(pattern.as_str()).unwrap();

        if re.is_match(&foo) {
            return true;
        }
    }

    return false;
}


fn first(filename:&str) {
    let input = get_file_data(filename);

    let mut sum:i64 = 0;
    
    let ranges = input.split(",");

    for range in ranges {
        let ends:Vec<&str> = range.split("-").collect();


        let low:i64  = match FromStr::from_str(ends[0].trim()) {
            Ok(num) => num,
            Err(_) => panic!("jotain kusi"),
        };
        let high:i64  = match FromStr::from_str(ends[1].trim()) {
            Ok(num) => num,
            Err(_) => panic!("jotain kusi"),
        };
        
        println!("Range: {low} - {high}:");
        for num in low..=high {
            if check_code(num) {
                println!("{num}");
                sum += num; 
            }
        }
    }



    println!("Final: {sum}");

}

fn second(filename:&str) {
    let input = get_file_data(filename);

    let mut sum:i64 = 0;
    
    let ranges = input.split(",");

    for range in ranges {
        let ends:Vec<&str> = range.split("-").collect();


        let low:i64  = match FromStr::from_str(ends[0].trim()) {
            Ok(num) => num,
            Err(_) => panic!("jotain kusi"),
        };
        let high:i64  = match FromStr::from_str(ends[1].trim()) {
            Ok(num) => num,
            Err(_) => panic!("jotain kusi"),
        };
        
        println!("Range: {low} - {high}:");
        for num in low..=high {
            if check_code2(num) {
                println!("{num}");
                sum += num; 
            }
        }
    }

    println!("Final: {sum}");

}




fn main() {
    first("test.txt");
    println!("----------");
    second("input.txt");
}
