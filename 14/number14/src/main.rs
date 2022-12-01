use std::fs;
use std::collections::HashMap;
use std::time::{Instant};

fn main() {
    
    let input = read_input("test_input.txt");

   // first(&input);
    let now = Instant::now();
    second(&input);
    println!("Time: {}ms", now.elapsed().as_millis());
}

fn second(input: &String){
    let polymer = get_polymer_template(input);
    let insertion_rules = get_pair_insertion_rules(input);
    println!("Poly. start: {}", polymer);

    let ans = process_and_count(&polymer, &insertion_rules, 40);
    println!("Ans: {}", ans);
}

fn process_and_count(polymer: &String, rules: &HashMap<String, String>, steps: i32) -> i64{
    let mut pair_count = get_pair_count_hashmap(rules);
    let mut temporary_pair_count = get_pair_count_hashmap(rules);
    let mut element_count_map = get_initial_element_map(polymer);

    //**** Setup
    let mut pair_cache: Vec<String> = Vec::new();
    let chars = polymer.chars();
    for (index, c) in chars.enumerate(){
        if index > 0{
            let len = pair_cache.len();
            pair_cache[len-1].push_str(&c.to_string());
        }
        pair_cache.push(c.to_string());

    }
    pair_cache.pop();

    for pair in pair_cache{
        *pair_count.entry(pair).or_insert(0) += 1;
    }

    let rule_result_pairs = get_rule_result_pair(rules);
    //****

    for _ in 0..steps{
        temporary_pair_count = reset_count_hashmap(&temporary_pair_count);

        for (key, value) in &pair_count{
            let pair1 = &rule_result_pairs.get(key).unwrap()[0];
            let pair2 = &rule_result_pairs.get(key).unwrap()[1];
            temporary_pair_count.insert(pair1.to_string(), temporary_pair_count[pair1]+value);
            temporary_pair_count.insert(pair2.to_string(), temporary_pair_count[pair2]+value);
            let added_element = rules.get(key).unwrap();
            *element_count_map.entry(added_element.to_string()).or_insert(0) += value;
        }

        pair_count = HashMap::clone(&temporary_pair_count);
    }

    do_it2(&pair_count);

    do_it2(&element_count_map);

    let mut min = i64::MAX;
    let mut max = i64::MIN;

    for (_, value ) in element_count_map{
        min = std::cmp::min(value, min);
        max = std::cmp::max(value, max);
    }

    max-min
}

fn get_initial_element_map(polymer: &String) -> HashMap<String, i64>{
    let mut res = HashMap::new();

    let chars = polymer.chars();
    for c in chars{
        let letter = c;
        *res.entry(letter.to_string()).or_insert(0) += 1;
    }
    res
}


fn get_rule_result_pair(rules: &HashMap<String,String>) -> HashMap<String,Vec<String>>{
    let mut res = HashMap::new();

    for (key, value) in rules{
        let mut chars = key.chars();
        let new = value;
        let first = chars.nth(0).expect("Unwrapping went wrong").to_string();
        let second = chars.nth(0).expect("Unwrapping went wrong").to_string();

        let mut first_pair = String::clone(&first);
        first_pair.push_str(new);
        let mut second_pair = String::clone(new);
        second_pair.push_str(&second);

        let pair_vec = vec![first_pair, second_pair];

        res.insert(String::clone(key), pair_vec);

    }

    res
}

fn get_pair_count_hashmap(rules: &HashMap<String, String>) -> HashMap<String, i64>{
    let mut res = HashMap::new();
    for (key, _) in rules{
        res.insert(String::clone(key), 0);
    }
    res
}

fn reset_count_hashmap(hashmap: &HashMap<String, i64>) -> HashMap<String, i64>{
    let mut res = HashMap::clone(hashmap);
    for (key, _) in hashmap{
        res.insert(key.to_string(), 0);
    }
    res
}


fn first(input: &String){
    let mut polymer = get_polymer_template(input);
    println!("Pol. start: {}", polymer);
    let insertion_rules = get_pair_insertion_rules(input);
    // do_it(&mut insertion_rules);

    polymer = get_processed_polymer(&polymer, &insertion_rules, 10);
    println!("len: {}", polymer.len());
    
    let ans = get_answer(&polymer);
    println!("Ans: {}", ans);
}

fn get_answer(polymer: &String) -> i32{
    let mut counts: HashMap<char, i32> = HashMap::new();
    let find = counts.contains_key(&'F');
    println!("Find: {}",find);
    
    let chars = polymer.chars();
    for c in chars{
        *counts.entry(c).or_insert(0) += 1;
    }

    let mut max = i32::MIN;
    let mut min = i32::MAX;

    for (key, value) in counts{
        println!("Key: {}. Value: {}", key, value);
        max = std::cmp::max(value, max);
        min = std::cmp::min(value, min);
    }

    println!("Max: {}, min: {}",max,min);
    max-min
}

fn get_processed_polymer(polymer: &String, rules: &HashMap<String, String>, steps: i32) -> String{
    let mut pol = String::clone(polymer);

    for _ in 0..steps{
        pol = get_processed_polymer_util(&pol, rules);
    }

    pol
}

fn get_processed_polymer_util(polymer: &String, rules: &HashMap<String,String>) -> String{
    let mut result = String::new();
    let mut pair_cache: Vec<String> = Vec::new();
    let chars = polymer.chars();
    for (index, c) in chars.enumerate(){
        if index > 0{
            let len = pair_cache.len();
            pair_cache[len-1].push_str(&c.to_string());
        }
        if  index == 0{
            result.push_str(&c.to_string());
        }
        pair_cache.push(c.to_string());

    }
    pair_cache.pop();

    
    for pair in pair_cache{
        let to_add = rules.get(&pair).unwrap();
        let mut chars = pair.chars();
        chars.nth(0);
     //   result.push_str(&chars.nth(0).unwrap().to_string());
        result.push_str(&to_add);
        result.push_str(&chars.nth(0).unwrap().to_string());
     //   println!("Res: {}", result);
    }

    result
}

fn get_pair_insertion_rules(input: &String) -> HashMap<String, String>{
    let mut rules = HashMap::new();

    let mut lines = input.lines();
    lines.nth(0);
    lines.nth(0);

    for line in lines{
        let mut chars = line.chars();
        let pair = chars.nth(0).unwrap().to_string() + &chars.nth(0).unwrap().to_string();
        let value = chars.nth(4).unwrap().to_string();
        rules.insert(pair,value);
    }

    rules
}

fn get_polymer_template(input: &String) -> String{
    let mut lines = input.lines();
    lines.nth(0).unwrap().to_string()
}

fn read_input(input: &str) -> String{
    let contents = fs::read_to_string(input)
            .expect("Something went wrong reading the file");
        
    contents.to_string()
}

fn do_it2(map: &HashMap<String, i64>) {
    for (key, value) in map {
        println!();
        print!("Key: {}. Value: {}", key, value);
    }
    println!();
}