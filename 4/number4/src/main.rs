use std::{fs};
use ansi_term::Style;
use ansi_term::Colour::{Red};

fn main() {
    let input = read_input("input.txt");
  //  first(&input);
    second(&input);
}

fn second(input: &String){
    let mut v: Vec<&str> = Vec::new();
    let lines = input.lines();
    for line in lines{
        v.push(line);
    }

    let picked_numbers: Vec<i32> = v[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let mut super_array = transform_lines_to_super_array(&v);

    // for number in picked_numbers{
    //     //super_array = update_matrix_for_bingo_number(super_array, number)
    // }
    let mut counter = 0;
    let number_of_matrixes = super_array.len();

    let mut hit = check_matrix_for_bingo(&mut super_array);
    let mut LastHit = check_matrix_for_bingo(&mut super_array);
    let mut last_hit_number = -1;



    for i in 0..picked_numbers.len(){
        let number = picked_numbers[i];
        super_array = update_matrix_for_bingo_number(super_array, number);
        hit = check_matrix_for_bingo(&mut super_array);
        if hit.0 == true{
           LastHit = (hit.0, Vec::clone(&hit.1));
            counter = counter + 1;
           // println!("HIT! i: {}", i);
           print_matrix(&hit.1);
            last_hit_number = number;
            if counter == number_of_matrixes{
                let unmarked_sum = get_sum_of_unmarked_numbers(&hit.1);
                println!("Sum1: {}", unmarked_sum);
                println!("End value1: {}", number*unmarked_sum);
                break;
            }

        }
    }

    print_matrix(&LastHit.1);
    let unmarked_sum = get_sum_of_unmarked_numbers(&LastHit.1);
    println!("Sum: {}", unmarked_sum);
    println!("Last number marked: {}", last_hit_number);
    println!("End value: {}", last_hit_number*unmarked_sum);
    
    println!();

    for i in 0..super_array.len(){
        println!("{}", super_array[i].0);
    }
    // for i in 0..super_array.len(){
    //     for j in 0..super_array[i].1.len(){
    //         for k in 0..super_array[i].1[j].len(){
    //             let value =super_array[i].1[j][k].1;
    //             let text = super_array[i].1[j][k].0.to_string();
    //             if value{
    //                 print!("{} ", Style::new().fg(Red).bold().paint(text));
    //             } else{
    //                 print!("{} ", text);
    //             }
                
    //         }
    //         println!();
    //     }
    //     println!();
    // }
}


fn first(input: &String){
    let mut v: Vec<&str> = Vec::new();
    let lines = input.lines();
    for line in lines{
        v.push(line);
    }

    let picked_numbers: Vec<i32> = v[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let mut super_array = transform_lines_to_super_array(&v);


    for i in 0..picked_numbers.len(){
        let number = picked_numbers[i];
        super_array = update_matrix_for_bingo_number(super_array, number);
        let hit= check_matrix_for_bingo(&mut super_array);
        if hit.0 == true{
        //    println!("HIT! i: {}", i);
            // print_matrix(&hit.0);
            print_matrix(&hit.1);
            let unmarked_sum = get_sum_of_unmarked_numbers(&hit.1);
            println!("Sum: {}", unmarked_sum);
            println!("End value: {}", number*unmarked_sum);
            break;
        }
    }
    
    // println!();
    // for i in 0..super_array.len(){
    //     for j in 0..super_array[i].len(){
    //         for k in 0..super_array[i][j].len(){
    //             let value =super_array[i][j][k].1;
    //             let text = super_array[i][j][k].0.to_string();
    //             if value{
    //                 print!("{} ", Style::new().fg(Red).bold().paint(text));
    //             } else{
    //                 print!("{} ", text);
    //             }
                
    //         }
    //         println!();
    //     }
    //     println!();
    // }


}

fn get_sum_of_unmarked_numbers(matrix: &Vec<Vec<(i32, bool)>>) -> i32{
    let mut total = 0;
    for row in matrix{
        for tuple in row{
            if tuple.1 == false{
                total += tuple.0;
            }
        }
    }
    total
}

fn check_matrix_for_bingo(all_data: &mut Vec<(bool, Vec<Vec<(i32, bool)>>)>) -> (bool, Vec<Vec<(i32, bool)>>){
    //let mut new_data = Vec::clone(&all_data);

    let mut res = (false, Vec::clone(&all_data[0].1));
    for i in 0..all_data.len(){
        let matrix = all_data.get_mut(i).unwrap();

        if matrix.0{
            continue;
        }
      //  let matrix = matrix.1;

        let mut column_totals = [true;5];
        for j in 0..matrix.1.len(){
            let row = matrix.1.get(j).unwrap();
            let row_total = 0;
            let mut row_true = true;
            for k in 0..row.len(){
                
                if row[k].1 == false{
                    column_totals[k] = false;
                    row_true = false;
                } 
            }
            if row_true{
                res = (true, Vec::clone(&matrix.1));
                matrix.0=true;
               // print_matrix(&res.1);
                break;
            }

        }
        for column in column_totals{
            if column == true{
                res = (true, Vec::clone(&matrix.1));
                matrix.0=true;
                break;
            }
        }
        
    }

    
    res
}

fn update_matrix_for_bingo_number(all_data: Vec<(bool, Vec<Vec<(i32, bool)>>)>, number: i32) -> Vec<(bool, Vec<Vec<(i32, bool)>>)>{

    let mut new_data = Vec::clone(&all_data);

    for i in 0..new_data.len(){
        let matrix_tuple = new_data.get_mut(i).unwrap();
        if matrix_tuple.0 == false{
            for j in 0..new_data.get_mut(i).unwrap().1.len(){
                let row = new_data.get_mut(i).unwrap().1.get_mut(j).unwrap();
                for k in 0..row.len(){
                    let temp_tuple = row.get_mut(k).unwrap();
                    if temp_tuple.0 == number{
                        temp_tuple.1 = true;
                    }
                }
            }
        }
        
    }
    new_data
}

fn transform_lines_to_super_array(v: &Vec<&str>) -> Vec<(bool, Vec<Vec<(i32, bool)>>)>{
    let mut super_array: Vec<(bool, Vec<Vec<(i32, bool)>>)> = Vec::new();
    let mut temp_arr: Vec<Vec<(i32, bool)>> = Vec::new();
    for i in 1..v.len(){
        let line = v[i];
        if line.len() == 0{
            if (temp_arr.len() > 0){
                super_array.push((false, Vec::clone(&temp_arr)));
            }
            temp_arr.clear();
            continue;
        }
        let mut line_numbers: Vec<i32> = line.split_whitespace().map(|x|x.parse::<i32>().unwrap()).collect();
        let mut sub_temp_array: Vec<(i32, bool)> = Vec::new();
        for number in line_numbers{
            let mut tuple = (number, false);
            sub_temp_array.push(tuple);
        }
        temp_arr.push(sub_temp_array);
    }
    super_array.push((false, Vec::clone(&temp_arr)));

    super_array
}

fn print_matrix(matrix: &Vec<Vec<(i32, bool)>>){
    println!();
    for row in matrix{
        for tuple in row{
            let value = tuple.1;
            let text = tuple.0.to_string();
            if value{
                print!("{} ", Style::new().fg(Red).bold().paint(text));
            } else{
                print!("{} ", text);
            }
          //  print!("{} ", tuple.0);
        }
        println!();
    }
}

fn read_input(input: &str) -> String{
    let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
    
    contents.to_string()
}