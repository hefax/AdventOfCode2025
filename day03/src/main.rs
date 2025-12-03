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
    let mut sum:i64=0;

    for (_,rivi) in input.lines().enumerate() {

        let row_r: Vec<i64> = rivi
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();

        let mut max_i = 0;
        let mut max = 0;

        for i in 0..row_r.len()-1 {
            if max < row_r[i] {
                max = row_r[i];
                max_i = i; 
            }
        }

        let mut next =0;
        let mut next_i=0;
        for i in max_i+1..row_r.len() {
            if next < row_r[i] {
                next = row_r[i];
                next_i = i;
            }
        }

        let jolt:i64 = 10*max+next;
    
        print!("[");
        for i in 0..row_r.len() {
            if i == max_i {
                print!("*{}*",row_r[i]);
            }
            else if i == next_i {
                print!("*{}*",row_r[i]);
            }
            else {
                print!("{}",row_r[i]);
            }
        }

        println!("] max jolt: {}",jolt);
        
        sum +=jolt;
    }


    println!("Final: {sum}");
}


fn main() {


    first("input.txt");

}
