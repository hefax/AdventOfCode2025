use std::fs::File;
use std::io::{self,BufRead};
use std::path::Path;
use std::str::FromStr;
use std::fmt;
use std::i64;
use std::collections::HashMap;
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


fn parse_stuff(input:String) {

    let pattern = format!(r"^\[([.#]*)\] (.*) \{{(.*)\}}");
    let pattern2 = format!(r"^\(([\d,]*)\)");

    let re = Regex::new(pattern.as_str()).unwrap();
    let re2 = Regex::new(pattern2.as_str()).unwrap();

    for (_,rivi) in input.lines().enumerate() {
        let parts = re.captures(rivi).unwrap();

        let leds = &parts[1];
        let power = &parts[3];

        let mut buttons:Vec<String> = vec![];

        let caps = re2.captures(&parts[2]).unwrap();

        for i in 0..caps.len() {

                /*
                match caps.get(i) {
                    Some(data) => {
                        buttons.push(data.map_or("", |m| m.as_str()));
                    },
                    None => {}
                }*/

            let s = caps.get(i).map_or("", |m| m.as_str());
            buttons.push(s.to_string());

        }

        println!("l: {:?} p: {:?} b: {:?}",leds,power,buttons);
    }

}



fn first(filename:&str) {
    let input = get_file_data(filename);

    parse_stuff(input);


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
    first("test.txt");
    second("test.txt");
}
