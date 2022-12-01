use std::{fs, collections::HashMap, borrow::BorrowMut, future::ready};
//use std::time::{Instant};

fn main() {

  //  let now = Instant::now();
    let input = read_input("input.txt");
    first(&input);

   // println!("Time: {}ms", now.elapsed().as_millis());
}

fn first(input: &String){
    let mut v = Vec::new();
    let lines = input.lines();
    for line in lines{
        v.push(line);
    }

    let cave_map: &mut HashMap<String, Cave> = &mut HashMap::new();

    for line in v {
     
        let mut split = line.split("-");
        let first = split.nth(0).unwrap();
        let second = split.nth(0).unwrap();


        if !cave_map.contains_key(second){
            let big_cave = is_upper_case(second);
            cave_map.insert(second.to_string(), Cave{is_small_cave: big_cave, neighbours: Vec::new()});
        }

        if cave_map.contains_key(first){
            let f = cave_map.get_mut(first).unwrap();
            f.neighbours.push(second.to_string());

            let t = cave_map.get_mut(second).unwrap();
            t.neighbours.push(first.to_string());

        } else {
            let big_cave = is_upper_case(first);
            cave_map.insert(first.to_string(), Cave{is_small_cave: big_cave, neighbours: Vec::new()});

            let f = cave_map.get_mut(first).unwrap();
            f.neighbours.push(second.to_string());

            let t = cave_map.get_mut(second).unwrap();
            t.neighbours.push(first.to_string());
        }
    }
    
    //do_it(cave_map);

    let all_paths = get_possible_paths(cave_map);
    println!("ap len: {}", all_paths.len());
}

fn get_possible_paths(map: &mut HashMap<String, Cave>) -> Vec<String>{
    let mut res = Vec::new();

    let start = map.get("start").unwrap();
    
    get_next(map, &mut res, &mut "start".to_string(), &mut "".to_string());

    res
}

fn get_next(map: &mut HashMap<String, Cave>, res: &mut Vec<String>, current: &String, rnr: &mut String){

    let running_string = &mut String::clone(rnr);
    if (*current).eq(&"start".to_string()) && running_string.len() > 0{
        return;
    }

    if (*current).eq(&"end".to_string()){
        let mut clone = String::clone(running_string);
        clone.push_str(&",end".to_string());
        res.push(clone);
        return
    }

    let curr = map.get(current).unwrap();

    if !is_upper_case(current){
        if path_contains_with_condition(running_string, &current){
            return
        }
    }   
    let mut nn = Vec::clone(&curr.neighbours);

    if (running_string.len() == 0){
        running_string.push_str(&"start".to_string());
    }else{
        running_string.push_str(&",".to_string());
        running_string.push_str(current);
    }

    for neighbour in &nn{
        get_next(map, res, &mut String::clone(neighbour), &mut String::clone(running_string));
    }
    

}

fn path_contains(path: &String, target: &String) -> bool{

    let path_clone = String::clone(path);

    let split =path_clone.split(",");
    for word in split{
        if target.eq(word){
            return true;
        }
    }

    false
}

fn path_contains_with_condition(path: &mut String, target: &String) -> bool{
    let path_clone = String::clone(path);
    let mut is_double_allowed = true;

    let split =path_clone.split(",");
    for (index,word) in split.enumerate(){
        if index == 0{
            if word.eq(&"double".to_string()){
                is_double_allowed = false;
            }
        }

        if target.eq(word){
            if is_double_allowed{
                let mut d = "double,".to_string();
                let content = String::clone(path);
                d.push_str(&content);

                *path = String::clone(&d);

                is_double_allowed = false;
            } else{
                return true;
            }
        }
    }

    false
}

fn do_it(map: &mut HashMap<String, Cave>) {
    for (key, value) in &*map {
        println!();
        print!("Key: {}. Neighbours: ", key);
        for neighbour in &value.neighbours{
            print!("{}, ",neighbour);
        }
    }
    println!();
}


fn is_upper_case(strng: &str) -> bool{
    let str_upper = strng.to_ascii_uppercase();

    if *strng != str_upper{
        return false;
    }    

    true
}

struct Path{
    is_small_cave_entered_twice: bool,
    path: String,
}
struct Cave{
    is_small_cave: bool,
    neighbours: Vec<String>,
}

fn read_input(input: &str) -> String{
    let contents = fs::read_to_string(input)
            .expect("Something went wrong reading the file");
        
        contents.to_string()
    }