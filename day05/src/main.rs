use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::str::FromStr;
// use std::collections::HashMap;
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

fn check_id(id:i64,ranges:&Vec<(i64,i64)>) -> bool{
    for range in ranges {
        if id >= range.0 && id <= range.1 {
            return true;
        }
    }

    false
}

fn first(filename:&str) {
    let input = get_file_data(filename);

    let mut ranges:Vec<(i64,i64)> = vec![];
    let mut product:Vec<i64> = vec![];
    
    let mut state="ranges";

    let mut fresh:i64 = 0;

    for (_,rivi) in input.lines().enumerate() {

        

        match state {
            "ranges" => {
                if rivi.trim().len() == 0 {
                    state="products";
                }
                else {
                    let ends:Vec<&str> = rivi.split("-").collect();
                    let low:i64  = match FromStr::from_str(ends[0].trim()) {
                        Ok(num) => num,
                        Err(_) => panic!("jotain kusi"),
                    };
                    let high:i64  = match FromStr::from_str(ends[1].trim()) {
                        Ok(num) => num,
                        Err(_) => panic!("jotain kusi"),
                    };


                    ranges.push((low,high));
                }
            },
            "products" => {

                let id:i64  = match FromStr::from_str(rivi.trim()) {
                    Ok(num) => num,
                    Err(_) => panic!("jotain kusi"),
                };

                if check_id(id,&ranges) {
                    fresh +=1;
                }
            },
            _ => panic!("wtf again?"),

        }



        
    }



    println!("Final: {fresh}");

}


fn check_range(low:i64,high:i64,ranges:mut &Vec<(i64,i64)>) -> bool{

    let mut low_t = low;
    let mut high_t = low;

    // set our ends in suitable distances. 
    for range in ranges {
        if low_t >= range.0 && low_t <= range.1 {
            // our low_range is in this range. 
            // we need to set our low end to the high end of this range.
            low_t = range.1+1;
        }

        if high_t >= range.0 && high_t <= range.1 {
            // our high_range is in this range. 
            // we need to set our high end to the low end of this range.
            low_t = range.0-1;
        }

    }
        

    if low_t > high_t {
        return false;
    }
    // we now have a range that is not conflicting on it's upper or lower bounds.
    // last thing to do is to check if there are smaller 
    // range(s) in our range. 
    //

    let mut gaps:Vec<(i64,i64)> = vec![];
    let mut gaps:Vec<(i64,i64)> = vec![];

    let mut gl:i64=-1;
    let mut gh:i64=-1;

    for range in ranges {
        if  range.0 >= low_t  && range.0 <= high {
            //this range low end is in our range .
            gl=range.0;
        }

        if  range.1 >= low_t  && range.1 <= high_t {
            // thie range high end is in our range.
            gh=range.1;
            assert!((gl > -1));
        }

        // note both of these should be set 
        //



        

    }

    false
}





fn second(filename:&str) {
    let input = get_file_data(filename);

    let mut ranges:Vec<(i64,i64)> = vec![];
    let mut product:Vec<i64> = vec![];
    
    let mut state="ranges";

    let mut fresh:i64 = 0;

    for (_,rivi) in input.lines().enumerate() {

        if rivi.trim().len() == 0 {
            state="products";
            break;
        }
        else {
            let ends:Vec<&str> = rivi.split("-").collect();
            let low:i64  = match FromStr::from_str(ends[0].trim()) {
                Ok(num) => num,
                Err(_) => panic!("jotain kusi"),
            };
            let high:i64  = match FromStr::from_str(ends[1].trim()) {
                Ok(num) => num,
                Err(_) => panic!("jotain kusi"),
            };


            ranges.push((low,high));
        }
    }


    let mut better:Vec<(i64,i64)> = vec![];

    for range in ranges {
        // 
        let mut low = range.0;
        let mut high = range.1;

        let mut low_t:i64 = -1;
        let mut high_t:i64 = -1;

        println!("Checking {low}-{high}");
        for i in low..=high {
            //print!("checking {i}");
            if check_id(i,&better) {
                if low_t < high_t {
                    better.push((low_t,high_t));
                }
                else {
                    if check_id(low_t,&better) == false && low_t > -1 {
                        better.push((low_t,low_t));
                    }
                }

                // we
                low_t=i+1;
                high_t=i+1;
                //println!("we were in range so set {low_t} - {high_t}");
            }
            else {
                if low_t == -1 {
                    low_t = i;
                }

                high_t = i;
                //println!("we were not in range so set {low_t} - {high_t}");
            }
        }

        if low_t < high_t {
            better.push((low_t,high_t));
        }
        else {
            if check_id(low_t,&better) == false && low_t > -1 {
                better.push((low_t,low_t));
            }
        }



    }

    // println!("{:?}",better);

    
    for range in better {

        fresh += range.1 - range.0 +1;

    }





    println!("Final: {fresh}");

}


fn main() {
    first("input.txt");
    second("input.txt");
}
