use std::fs::read_to_string;

static MAX_RED: i32 = 12;
static MAX_BLUE: i32 = 14;
static MAX_GREEN: i32 = 13;

fn check_isvalid(color: &str,number: i32)->bool{
    let max_check = match color {
        "blue"=>MAX_BLUE,
        "red"=>MAX_RED,
        "green"=>MAX_GREEN,
        _=>0,
    };
    if number > max_check {
        return false;
    } else {
        return true;
    };
        
}

fn main() {
    let mut sum:i32 = 0;

    let file = read_to_string("src/day2.txt").unwrap().to_string();
    for line in file.lines(){
        let mut valid = true;
        let mut _split: Vec<&str> = line.split(":").collect();
        let game_id = _split[0].split(" ").map(|f| f.trim()).collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let _split1  = _split[1].split(";");
        let _trim1 = _split1.map(|f| f.trim());
        for items in _trim1{
            valid = true;
            let mut _split2 = items.split(",").map(|f| f.trim()).map(|f| f.split(" "));
            for i in _split2{
                let _tmp2 = i.collect::<Vec<&str>>();
                valid = check_isvalid(_tmp2[1], _tmp2[0].parse::<i32>().unwrap());
                if !valid{
                    break;
                }
            }
            if !valid{
                break;
            }
        }
        if valid{
            sum+=game_id;
        }
    }
    println!("{}",sum);
}
