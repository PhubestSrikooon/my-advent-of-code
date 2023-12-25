use std::fs::{File , read_to_string};
use regex::Regex;

fn main(){
    let mut sum = 0;
    let re = Regex::new(r"[0-9]").unwrap();
    for _line in read_to_string("src/day1.txt").unwrap().lines(){
        let mut nums = Vec::new();
        for _char in _line.split(""){
            if re.is_match(_char.to_string().as_str()){
                nums.push(_char);
            }
        }
        let fom : i32 = format!("{}{}",nums[0],nums[nums.len()-1]).parse().unwrap();
        sum+=fom;
    }
    println!("{}",sum);
}