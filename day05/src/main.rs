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


fn check_range(low:i64,high:i64,ranges:&Vec<(i64,i64)>) -> Option<Vec<(i64,i64)>> {

    let mut low_t = low;
    let mut high_t = high;

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
            high_t = range.0-1;
        }

    }
        

    if low_t > high_t {
        return None;
    }
    // we now have a range that is not conflicting on it's upper or lower bounds.
    // last thing to do is to check if there are smaller 
    // range(s) in our range. 
    //

    let mut gaps:Vec<(i64,i64)> = vec![];
    gaps.push((low_t,high_t));

    
    if ranges.len() == 0 {
        return Some(gaps);
    }

    loop {
        let mut split = false;
        let my_range = gaps.pop();

        match my_range {
            Some(my_range) => {
                let mut tmp:Vec<(i64,i64)> = vec![];
                tmp.push(my_range);

                // shoot holes. into the range:
                for range in ranges {
                    let splits = inset_range(&tmp,&range);

                    if splits.len() > 1 {
                        split=true;
                    }

                    tmp = splits;

                }

                // not sure if duplicate filtering is needed anymore.
                for item in tmp {
                    let fo = gaps.iter().find(|&&val| val==item);

                    match fo {
                        Some(_) => { 
                            // we already have this;
                        },
                        None => {
                            gaps.push((item.0,item.1));
                        }
                    }
                }

            },
            None => {
            },
        }

        if split == false {
            break;
        }
    }
        

    Some(gaps)
}

fn inset_range(my_range:&Vec<(i64,i64)>,existing:&(i64,i64)) -> Vec<(i64,i64)> {
    let mut set:Vec<(i64,i64)> = vec![];
    let mut result:Vec<(i64,i64)> = vec![];


    for item in my_range {
        set.push((item.0,item.1));
    }
    let low_x = existing.0;
    let high_x = existing.1;


    loop {

        let range = set.pop();

        match range {
            Some(range) => {
                let mut low= range.0;
                let mut high= range.1;
                
                if high_x < low {
                    // the existing does no affect this part. 
                    result.push((low,high));
                    continue;
                }

                if low_x > high {
                    // the existing does no affect this part.
                    result.push((low,high));
                    continue;
                }

                if low_x > low && high_x < high {
                    // the existing is in the middle of this part. 
                    // lets just add this here 
                    set.push((low,low_x-1));
                    set.push((high_x+1,high));
                    continue;
                }


                if low_x == low && high_x < high {
                    low = high_x+1; 
                    set.push((low,high));
                    continue;

                }

                if low_x > low && high_x == high {
                    high = low_x-1;
                    set.push((low,high));
                    continue;

                }


                panic!("this should not be.");
            },
            None => {
                break;
            },
        }

    }


 //                    println!("{:?} - {:?} -> {:?}",my_range,existing,result);


    result
}




fn second(filename:&str) {
    let input = get_file_data(filename);

    let mut ranges:Vec<(i64,i64)> = vec![];

    let mut fresh:i64 = 0;

    for (_,rivi) in input.lines().enumerate() {

        if rivi.trim().len() == 0 {
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
        let  low = range.0;
        let  high = range.1;

        println!("Checking {low}-{high}");
        match check_range(low,high,&better) {
            Some(list) => {
                for item in list {
                    better.push((item.0,item.1));
                }
            }
            None => {},
        }

    }


    // println!("{:?}",better);

    for range in better {
        fresh += range.1-range.0+1;
    }


    println!("Final: {fresh}");

}


fn main() {
    first("input.txt");
    second("input.txt");
}
