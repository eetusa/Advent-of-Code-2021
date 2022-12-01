use std::fs;
use std::time::{Instant};

const CHARS: [char; 7] = ['a', 'b', 'c', 'd', 'e', 'f', 'g'];
fn main() {
    let now = Instant::now();
    let input = read_input("input.txt");
    second(&input);
    println!("Time: {}ms", now.elapsed().as_millis());
}

fn second(input: &String){
    let mut v: Vec<&str> = Vec::new();
    let lines = input.lines();
    for line in lines{
        v.push(line);
    }
   // let value = decode_line(v[0]);
   
    let mut sum = 0;
    for line in v{
        let value = decode_line(line);
        sum = sum + value;
    }   
    println!("Sum: {}", sum);
}

fn decode_line(line: &str)-> i32{
    let mut all_segments: Vec<&str> = Vec::new();
    let mut all_segments_numbers  = [-1, -1, -1, -1, -1, -1, -1, -1, -1, -1];
    let split = line.split_whitespace();

    // get all subsets
    for a in split{
        if a.len() > 1{
            let mut is_already_in = false;
            for i in 0..all_segments.len(){
                if are_subsets_equal(all_segments[i], a){
                    is_already_in = true;
                }
            }
            if !is_already_in{
                all_segments.push(a);
            }
        }
    }
    
    let mut runner: Vec<&str> = Vec::new();
    let mut zero = "";
    let mut one = "";
    let mut three = "";
    let mut four = "";
    let mut five = "";
    let mut seven = "";
    let mut eight = "";
    let mut nine = "";
    // 1, 4, 7, 8
    for i in 0..all_segments.len(){
        let segment = all_segments[i];
        let len = segment.len();
        if (len == 2){
            all_segments_numbers[i] = 1;
            one = segment;
            runner.push(segment);
        } else if (len == 4){
            all_segments_numbers[i] = 4;
            four = segment;
            runner.push(segment);
        } else if (len == 3){
            all_segments_numbers[i] = 7;
            seven = segment;
            runner.push(segment);
        } else if (len == 7){
            all_segments_numbers[i] = 8;
            runner.push(segment);
            eight = segment;
        }
    }

    // 9
    for i in 0..all_segments.len(){
        let segment = all_segments[i];
        let len = segment.len();
        if is_subset(segment, four) && len  == 6{
            all_segments_numbers[i] = 9;
            nine = segment;
            runner.push(segment);
        }
    }

    // 3
    for i in 0..all_segments.len(){
        let segment = all_segments[i];
        let len = segment.len();
        if is_subset(segment, one) && is_subset(nine, segment) && !runner.contains(&segment){ 
            all_segments_numbers[i] = 3;
            three = segment;
            runner.push(segment);
        }
    }

    // 5
    for i in 0..all_segments.len(){
        let segment = all_segments[i];
        let len = segment.len();
        if is_subset(nine, segment) && !are_subsets_equal(three, segment) && !runner.contains(&segment){
            all_segments_numbers[i] = 5;
            five = segment;
            runner.push(segment);
        }
    }

    // 0
    for i in 0..all_segments.len(){
        let segment = all_segments[i];
        let len = segment.len();
        if len == 6 && is_subset(segment, one)  && !runner.contains(&segment){
            all_segments_numbers[i] = 0;
            zero = segment;
            runner.push(segment);
        }
    }

    // 2
    for i in 0..all_segments.len(){
        let segment = all_segments[i];
        let len = segment.len();
        if len == 5 && !runner.contains(&segment){
            all_segments_numbers[i] = 2;
            runner.push(segment);
        }
    }

    // 6
    for i in 0..all_segments.len(){
        let segment = all_segments[i];
        let len = segment.len();
        if len == 6 && !runner.contains(&segment){
            all_segments_numbers[i] = 6;
            runner.push(segment);
        }
    }

    let mut number = "".to_string();
 
    let split = line.split("|").nth(1).unwrap().split(" ");
    for a in split{
        for i in 0..all_segments.len(){
            let compare_to = all_segments[i];
            if are_subsets_equal(a, compare_to){
                number.push_str(&all_segments_numbers[i].to_string());
                break;
            }
        }
    }

   // println!("Num: {}", number);

    // for i in 0..all_segments.len(){
    //     println!("Segment: {} , digit: {}", all_segments[i], all_segments_numbers[i]);
    // }
    number.parse().unwrap()
}

fn is_subset(superset: &str, subset: &str) -> bool{
    let subset_chars = subset.chars();

    for c in subset_chars{
        if !superset.contains(c){
            return false;
        }
    }
    
    true
}

fn are_subsets_equal(str1: &str, str2: &str) -> bool{
    let char_arr1 = str1.chars();
    let char_arr2 = str2.chars();

    let mut str1_vec = [0;7];
    let mut str2_vec = [0;7];

    for c in char_arr1{
        let index = CHARS.iter().position(|&r| r == c).unwrap();
        str1_vec[index] = str1_vec[index] + 1;
    }
    for c in char_arr2{
        let index = CHARS.iter().position(|&r| r == c).unwrap();
        str2_vec[index] = str2_vec[index] + 1;
    }

    for i in 0..str1_vec.len(){
        if str1_vec[i] != str2_vec[i]{
            return false;
        }
    }
    
    true
}

fn first(input: &String){
    let mut v: Vec<&str> = Vec::new();
    let lines = input.lines();
    for line in lines{
        v.push(line);
    }

    let mut numbers: Vec<Vec<&str>> = Vec::new();
    for _ in 0..10{
        numbers.push(Vec::new());
    }

    for line in v{
        let split = line.split("|").nth(1).unwrap().split(" ");
        for a in split{
            let len = a.len();
            if len == 2 || len == 3 || len == 4 || len == 7{
                numbers[len].push(a);
            } 
        }
    }

    let mut sum = 0;

    for vec in numbers{
        for _ in vec{
            sum = sum + 1;
        }
    }

    println!("Sum: {}", sum);
}

fn read_input(input: &str)-> String{
    let contents = fs::read_to_string(input).expect("Something went wrong with reading the file.");
    contents.to_string()
}