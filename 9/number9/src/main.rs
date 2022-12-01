use std::fs;
use std::time::{Instant};

fn main() {

    let now = Instant::now();
    let input = read_input("input.txt");
    first(&input);
    println!("Time: {}ms", now.elapsed().as_millis());
}

fn first(input: &String){
    let matrix = get_matrix_from_string(&input);
    
    let low_points = get_low_points(&matrix);
    let risk_sum = get_risk_level_sum(&matrix, &low_points);


    println!("Sum: {}", risk_sum);

    let basins = get_basins_from_low_points(&matrix, &low_points);
    let multiplication_of_basins = get_multiplication_of_three_biggest_basins(&basins);

    println!("Multiplication: {}", multiplication_of_basins);
}

fn get_multiplication_of_three_biggest_basins(basins: &Vec<Vec<(i32,i32)>>) -> usize{
    let mut lengths = Vec::new();
    for basin in basins{
        lengths.push(basin.len());
    }
    lengths.sort();
    lengths.reverse();

    let multiplication = lengths[0]*lengths[1]*lengths[2];
    multiplication
}

fn get_basins_from_low_points(matrix: &Vec<Vec<i32>>, low_points: &Vec<(i32,i32)>) -> Vec<Vec<(i32,i32)>>{
    let mut res = Vec::new();

    for point in low_points{
        let basin = get_single_basin_from_low_point(matrix, point);
        res.push(basin);
    }

    res
}

fn get_single_basin_from_low_point(matrix: &Vec<Vec<i32>>, low_point_pos: &(i32,i32)) -> Vec<(i32,i32)>{
    let mut res = Vec::new();
    basin_util(matrix,low_point_pos, &mut res);
    res
}

fn basin_util(matrix: &Vec<Vec<i32>>, current_pos: &(i32,i32), current_basin: &mut Vec<(i32,i32)>){
    current_basin.push(*current_pos);
    let x = current_pos.1 as usize;
    let y = current_pos.0 as usize;
    let cols = matrix[0].len();
    let rows = matrix.len();
    let value = matrix[y][x]; // double check
    let mut row_to_check_against: Vec<i32> = Vec::new();
    let mut col_to_check_against: Vec<i32> = Vec::new();

    if y != 0{
        row_to_check_against.push(-1);
    }
    if y != rows - 1 {
        row_to_check_against.push(1);
    }
    if x != 0{
        col_to_check_against.push(-1);
    }
    if x != cols - 1 {
        col_to_check_against.push(1);
    }

    for idx in row_to_check_against{
        let compare =matrix[(y as i32+idx) as usize][x];
        if compare == 9{
            continue;
        }
        if value < compare{
            let tuple =( y as i32+idx, x as i32);
            if !has_tuple(current_basin, tuple){
                basin_util(matrix, &tuple, current_basin);
            }
        }
    }

    for idx in col_to_check_against{
        let compare =matrix[y][(x as i32+idx) as usize];
        if compare == 9{
            continue;
        }
        if value < compare{
            let tuple =(y as i32, x as i32+idx);
            if !has_tuple(current_basin, tuple){
                basin_util(matrix, &tuple, current_basin);
            }
        }
    }
}

fn has_tuple(basin: &Vec<(i32,i32)>, tuple: (i32, i32)) -> bool{
    let t1 = tuple.0;
    let t2 = tuple.1;
    for i in 0..basin.len(){
        let v1 = basin[i].0;
        let v2 = basin[i].1;
        if t1 == v1 && t2 == v2{
            return true;
        }        
    }
    false
}

fn get_risk_level_sum(matrix: &Vec<Vec<i32>>, coordinates: &Vec<(i32,i32)>) -> i32{
    let mut sum = 0;
    for point in coordinates{
        let value = matrix[point.0 as usize][point.1  as usize] + 1;
        sum = sum + value;
    }
    sum
 }


fn get_matrix_from_string(input: &String) -> Vec<Vec<i32>>{
    let mut res: Vec<Vec<i32>> = Vec::new();

    let rows = input.lines();
    for row in rows{
        let mut v: Vec<i32> = Vec::new();
        let c_arr =row.chars();
        for n in c_arr{
            v.push(n.to_digit(10).unwrap() as i32);
        }
        res.push(v);
    }
    res
}

 fn get_low_points(matrix: &Vec<Vec<i32>>) -> Vec<(i32,i32)>{
    let mut res = Vec::new();

    let cols = matrix[0].len();
    let rows = matrix.len();

    for i in 0..rows{
        for j in 0..cols{
            let value = matrix[i][j]; // double check
            let mut row_to_check_against: Vec<i32> = Vec::new();
            let mut col_to_check_against: Vec<i32> = Vec::new();
            if i != 0{
                row_to_check_against.push(-1);
            }
            if i != rows - 1 {
                row_to_check_against.push(1);
            }
            if j != 0{
                col_to_check_against.push(-1);
            }
            if j != cols - 1 {
                col_to_check_against.push(1);
            }
            let mut is_a_low_point = true;

            for idx in row_to_check_against{

                if value >= matrix[(i as i32+idx) as usize][j]{
                    is_a_low_point = false;
                }
            }
            for idx in col_to_check_against{

                if value >= matrix[i][(j as i32+idx) as usize]{
                    is_a_low_point = false;
                }
            }
            if is_a_low_point{
                res.push((i as i32,j as i32));
            }
        }
    }
    res
 }


fn read_input(input: &str) -> String{
let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
    
    contents.to_string()
}