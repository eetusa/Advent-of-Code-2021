use std::{fs};
use std::time::{Instant};
fn main() {
    let now = Instant::now();
    let input = read_input("input.txt");
    first(&input);
    println!("{}", now.elapsed().as_millis());
}

fn first(input: &String){
    let mut positions: Vec<i32> = input.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let shortest_pos = get_shortest_position_to_move(&mut positions);
    println!("Best point: {}", shortest_pos);
}

fn get_shortest_position_to_move(vec: &mut Vec<i32>) -> i32{
    
    let mut all_sums = Vec::new();

    vec.sort();

    let mut min_sum = i32::MAX;
    let min = vec[0];
    let max = vec[vec.len()-1];

    println!("Min: {}, Max: {}", min, max);
    let mut best_point = 0;

    for i in min..max+1{
        let distance_sum = get_total_distance_from_points_to_point(&vec, i);
        all_sums.push(distance_sum);
        if distance_sum <= min_sum{
            min_sum = distance_sum;
            best_point = i;
        }
    }

    println!("Smallest sum: {}", min_sum);
    best_point
}

fn get_total_distance_from_points_to_point(vec: &Vec<i32>, target: i32) -> i32{
    let mut sum = 0;

    for point in vec{
        // let mut distance = 0;
        // for i in 1..(*point-target).abs()+1{
        //     distance = distance + i;
        // }
        let n = (*point-target).abs(); // 89791146
        let distance = n*(n+1)/2;
        sum = sum + distance;
    }

    sum
}

fn read_input(input: &str)-> String{
    let contents = fs::read_to_string(input).expect("Something went wrong with reading the file.");
    contents.to_string()
}