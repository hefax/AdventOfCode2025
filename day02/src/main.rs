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

fn check_code(id:i64) -> bool {
    
    let foo = id.to_str();



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
        for num in low..high {
            println!("{num}");
        }
    }



    println!("Final: {sum}");

}




fn main() {
    first("test.txt");
}
