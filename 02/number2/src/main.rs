use std::fs;
fn main() {
    let filename = "src\\input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let test_input = fs::read_to_string("src\\test_input.txt")
        .expect("Something went wrong reading the file");

   // first(contents);
    second(contents);
}

fn second(input: String){
    let mut _v: Vec<&str> = Vec::new();

    let lines = input.lines();

    for line in lines{
        _v.push(line);
    }

    let mut aim = 0;
    let mut depth = 0;
    let mut forward= 0;

    for i in 0.._v.len(){
        let line  = _v[i];
        let split = line.split(" ");

        let split_vec = split.collect::<Vec<&str>>();
        let amount = split_vec[1].parse::<i32>().unwrap();
        if split_vec[0] == "forward"{
            forward += amount;
            depth += aim*amount;
        } else if split_vec[0] == "down"{
            aim += amount;
        } else {
            aim -= amount;
        }
    }

    println!("Forward: {}", forward);
    println!("Depth: {}", depth);
    println!("Multiplied: {}", forward*depth);
}

fn first(input: String){
    let mut _v: Vec<&str> = Vec::new();

    let lines = input.lines();

    for line in lines{
        _v.push(line);
    }

    let mut depth = 0;
    let mut forward= 0;

    for i in 0.._v.len(){
        let line  = _v[i];
        let split = line.split(" ");

        let split_vec = split.collect::<Vec<&str>>();
        let amount = split_vec[1].parse::<i32>().unwrap();
        if split_vec[0] == "forward"{
            forward += amount;
        } else if split_vec[0] == "down"{
            depth += amount;
        } else {
            depth -= amount;
        }
    }

    println!("Forward: {}", forward);
    println!("Depth: {}", depth);
    println!("Multiplied: {}", forward*depth);
}