use std::fs;
use std::collections::HashMap;
use std::ops::RangeBounds;
use std::time::{Instant};

fn main() {
    let input = read_input("input.txt");
    second(&input);
}

fn first(input: &String){
    let mut v: Vec<&str> = Vec::new();
    let lines = input.lines();
    for line in lines{
        v.push(line);
    }
    let illegal_characters_indexes = get_illegal_character_indexes_per_row(&v);
    for x in &illegal_characters_indexes{
        if x.len() == 0{
            print!("Empty");
        }
        for a in x{
            print!("{}, ", a);
        }
        println!();
    }
    let sum = get_points_per_first_illegal_per_line(&v, &illegal_characters_indexes);

    println!("End sum: {}", sum);
}


fn second(input: &String){
    let mut v: Vec<&str> = Vec::new();
    let lines = input.lines();
    for line in lines{
        v.push(line);
    }

    let illegal_characters_indexes = get_illegal_character_indexes_per_row(&v);
    discard_illegal_lines(&mut v, &illegal_characters_indexes);
    let corrected_lines = correct_lines(&v);

    let mut points = get_points_from_closing_brackets(&corrected_lines);
    
    let answer = get_median_from_list(&mut points);
    println!("Answer: {}", answer);
}

fn get_median_from_list(list: &mut Vec<i64>) -> i64{
    let len = list.len();
    let halfpoint = len/2;
    list.sort();
    let res = list[halfpoint];
    return res
}

fn get_points_from_closing_brackets(brackets: &Vec<String>) -> Vec<i64>{
    let mut res = Vec::new();

    let mut point_set = HashMap::new();
    point_set.insert(')', 1);
    point_set.insert(']', 2);
    point_set.insert('}', 3);
    point_set.insert('>', 4);

    for str in brackets{
        let mut sum = 0;
        let chars = str.chars();

        for c in chars{
            let char_value = point_set.get(&c).unwrap();
            sum = sum * 5;
            sum = sum + char_value;
        }
        res.push(sum);
    }

    res
}

fn correct_lines(vector: &Vec<&str>) -> Vec<String>{
    let mut string_vector: Vec<String> = Vec::new();
    for s in vector{
        string_vector.push(add_correct_characters_to_end(s));
    }

    string_vector
}

fn discard_illegal_lines(vec: &mut Vec<&str>, illegal_chars: &Vec<Vec<i32>>){
    for i in (0..illegal_chars.len()).rev(){
        let x = &illegal_chars[i];
        if x.len() != 0{
            vec.remove(i);
        }

    }
}

fn get_illegal_character_indexes_per_row(input: &Vec<&str>) -> Vec<Vec<i32>>{
    let mut res = Vec::new();
    for line in input{
        res.push(get_illegal_character_indexes_on_row(line));
    }

    res
}

fn add_correct_characters_to_end(row: &str) -> String{
    let opens = ['(', '{', '<', '['];
    let mut closing_queue = Vec::new();
    let mut next_closing_char = 'x';
    let chars = row.chars();
    for c in chars{
        if opens.contains(&c){
            next_closing_char = get_next_closing_bracket_char(c);
            closing_queue.push(next_closing_char);
        } else {
            closing_queue.pop();
        }
    }
    let mut temp = String::new();
    for i in (0..closing_queue.len()).rev(){
        temp.push(closing_queue[i]);
    }

    temp
}

fn get_illegal_character_indexes_on_row(row: &str) -> Vec<i32>{
    let mut res = Vec::new();
    let opens = ['(', '{', '<', '['];
    let mut closing_queue = Vec::new();
    let mut next_closing_char = 'x';
  //  println!("xxxxxx");
    let chars = row.chars();
    for (index, c) in chars.enumerate(){
     //   println!("Char: {}",c);
        if opens.contains(&c){
            next_closing_char = get_next_closing_bracket_char(c);
          //  println!("Push index: {}", index);
         //   print_vec_as_line(&closing_queue);
            closing_queue.push(next_closing_char);
       //     print_vec_as_line(&closing_queue);
        } else {
           // println!("Pop index: {}", index);
            if next_closing_char != c{
                res.push(index as i32);
            }
          //  print_vec_as_line(&closing_queue);
            closing_queue.pop();
            if closing_queue.len() != 0{
                next_closing_char = closing_queue[closing_queue.len()-1];
            } else {
                next_closing_char = 'x';
            }

          //  print_vec_as_line(&closing_queue);
            
        }
    }
 //   println!("Closing queue: ");
   // print_vec_as_line(&closing_queue);
    res
}

fn print_vec_as_line(vec: &Vec<char>){
    print!("Queue: ");
    for c in vec{
        print!("{},", c);
    }
    println!();
}

fn get_next_closing_bracket_char(c: char) -> char{
    let opens = ['(', '{', '<', '['];
    let closes= [')', '}', '>', ']'];

    let index = opens.iter().position(|x| x == &c).unwrap();
    let res = closes[index];

    res
}

fn get_points_per_first_illegal_per_line(vector: &Vec<&str>, indexes: &Vec<Vec<i32>>) -> i32{
    if indexes.len() != vector.len(){
        return 0;
    }
    let mut point_set = HashMap::new();
    point_set.insert(')', 3);
    point_set.insert(']', 57);
    point_set.insert('}', 1197);
    point_set.insert('>', 25137);
    let mut sum = 0;

    for i in 0..vector.len(){
        if indexes[i].len() == 0{
            continue;
        }

        let row = vector[i];
        let index = indexes[i][0];
        let chars = row.chars();
        let char = row.chars().nth(index as usize).unwrap();
        let value = point_set.get(&char).unwrap();
        sum = sum + value;
    }
    
    sum as i32
}

fn read_input(input: &str)-> String{
    let contents = fs::read_to_string(input).expect("Something went wrong with reading the file.");
    contents.to_string()
}