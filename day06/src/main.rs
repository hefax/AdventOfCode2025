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



fn first(filename:&str) {
    let input = get_file_data(filename);

    let mut sum:i64 = 0;
    
    let mut nums:Vec<Vec<i64>> = vec![];

    let mut operators:Vec<&str> = vec![];

    for (_,rivi) in input.lines().enumerate() {
        let re = Regex::new(r"[\d*+]+").unwrap();

        let set:Vec<&str> = re.find_iter(rivi)
            .filter_map(|digit| Some(digit.as_str()))
            .collect();
    

        if nums.len() < set.len() {
            let missing = set.len()-nums.len();

            for _i in 0..missing {
                println!("Adding missing");
                let tmp:Vec<i64> = vec![];
                nums.push(tmp);
            }
        }


        for (i,item) in set.iter().enumerate() {
            match *item {
                "*" => {
                    // operator for this column is *
                    operators.push("*");
                },
                "+" => {
                    // operator for this column is +
                    operators.push("+");
                },
                _ => {

                    let num:i64  = match FromStr::from_str(item) {
                        Ok(num) => num,
                        Err(_) => panic!("jotain kusi"),
                    };

                    nums[i].push(num);
                    // lets assume this is still part of the 
                    // number list. soooo..
                }
            }
        }
    }
    for i in 0..nums.len() {
        let mut tmp:i64=0;
        match operators[i] {
            "*" => {
                for num in &nums[i] {
                    if tmp == 0 {
                        tmp = 1;
                    }
                    tmp *= num;
                }
                println!("multiply {:?} = {} ",nums[i],tmp);
            },
            "+" => {
                for num in &nums[i] {
                    tmp += num;
                }
                println!("sum {:?} = {} ",nums[i],tmp);
            },
            _ => {
                panic!("wtf?");
            }
        }
        sum += tmp;
    }
    println!("Final: {sum}");

}



fn second(filename:&str) {
    let input = get_file_data(filename);

    let mut sum:i64 = 0;
    
    // let ranges = input.split_whitespace();

    let mut chars:Vec<Vec<char>> = vec![];


    for (_,rivi) in input.lines().enumerate() {
        let clist:Vec<char> = rivi.chars().collect();
        chars.push(clist);
    }

    let mut nums:Vec<i64> = vec![];
    for x in (0..chars[0].len()).rev() {

        let mut tmp:i64= 0;
        let mut operator = 'x';
        let mut valid = false;
        for y in 0..chars.len() {
            match chars[y][x] {
                ' ' => {},
                '+' => {
                    // we finally have and operator for this set. 
                    operator = '+';
                },
                '*' => {
                    // we finally have and operator for this set. 
                    operator = '*';

                }
                _ => {
                    valid=true;
                    let number:i64 = chars[y][x].to_digit(10).unwrap() as i64;

                    tmp = tmp*10 + number;
                }
            }
        }
        
        if operator == 'x' {
            if valid == true {
                // just add the number into the 
                nums.push(tmp);
            }
        }
        else {
            if valid == true {
                // add the last number into the 
                nums.push(tmp);
            }

            tmp = 0;
            match operator {
                '*' => {
                    for num in &nums {
                        if tmp == 0 {
                            tmp = 1;
                        }
                        tmp *= num;
                    }
                    println!("multiply {:?} = {} ",nums,tmp);
                },
                '+' => {
                    for num in &nums {
                        tmp += num;
                    }
                    println!("sum {:?} = {} ",nums,tmp);
                },
                _ => {
                    panic!("wtf?");
                }
            }
            nums =vec![];
            sum += tmp;
        }

    }

    println!("Final: {sum}");

}


fn main() {
    first("test.txt");
    second("input.txt");
}
