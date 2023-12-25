use std::fs::read_to_string;
use std::cmp::max;


fn main() {
    let mut sum:i32 = 0;

    let file = read_to_string("src/day2.txt").unwrap().to_string();
    for line in file.lines(){
        let mut max_red = 0;
        let mut max_blue=0;
        let mut max_green=0;
        let mut _split: Vec<&str> = line.split(":").collect();
        // let game_id = _split[0].split(" ").map(|f| f.trim()).collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let _split1  = _split[1].split(";");
        let _trim1 = _split1.map(|f| f.trim());
        for items in _trim1{
            let mut _split2 = items.split(",").map(|f| f.trim()).map(|f| f.split(" "));
            for i in _split2{
                let _tmp2 = i.collect::<Vec<&str>>();
                match _tmp2[1] {
                    "red"=>{
                        max_red = max(max_red, _tmp2[0].parse::<i32>().unwrap());
                    },
                    "green"=>{
                        max_green = max(max_green, _tmp2[0].parse::<i32>().unwrap());
                    },
                    "blue"=>{
                        max_blue = max(max_blue, _tmp2[0].parse::<i32>().unwrap());
                    },
                    &_=>(),
                };
            }
        }
        // sum of each line is min_red*min_green*min_blue
        sum+=max_blue*max_green*max_red;
    }
    println!("{}",sum);
}
