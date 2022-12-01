use std::fs;
fn main() {
    let input = read_input("input.txt");
    first(&input);
}

fn first(input: &String){
    let mut v: Vec<&str> = Vec::new();
    let lines = input.lines();
    for line in lines{
        v.push(line);
    }

    let mut matrix = get_matrix_from_string(&input);
    let days = 351;
    let amount_of_flashes = simulate_steps(&mut matrix, days);

    for row in &matrix{
        for value in row{
            print!("{}",value.value);
        }
        println!();
    }
    println!();
    println!("Flashes: {}", amount_of_flashes);

}


fn simulate_one_step(matrix: &mut Vec<Vec<Octopus>>) -> i32{
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut flashes = 0;

    for i in 0..rows{
        for j in 0..cols{
            let value = matrix[i][j].value;
            matrix[i][j].value = value + 1;
        }
    }

    for i in 0..rows{
        for j in 0..cols{
            let value = matrix[i][j].flashed_this_iteration;
            if !value && matrix[i][j].value > 9{
                flash_substep(matrix, i, j, &mut flashes);
            }
        }
    }

    for i in 0..rows{
        for j in 0..cols{
            let value = matrix[i][j].value;
            matrix[i][j].flashed_this_iteration = false;
            if value > 9{
                matrix[i][j].value = 0;
            }
        }
    }
    flashes
}

fn flash_substep(matrix: &mut Vec<Vec<Octopus>>, i: usize, j: usize, flashes: &mut i32){

    let rows = matrix.len() as i32;
    let cols = matrix[0].len() as i32;
    let d:[i32; 3] = [-1,0,1];
    matrix[i][j].value = matrix[i][j].value +1;
    matrix[i][j].flashed_this_iteration = true;
    *flashes = *flashes + 1;


    for k in 0..d.len(){
        let dy = i as i32 + d[k];
        if dy >= 0 && dy < rows{
            for l in 0..d.len(){
                let dx = j as i32 + d[l];
                if k == 1 && l == 1{
                    continue;
                }
                if dx >= 0 && dx < cols{
                    let value = matrix[dy as usize][dx as usize].value;
                  //  println!("i: {}, j: {}, value: {}, dx: {}, dy: {}, target value: {}", i, j, matrix[i][j], dx, dy, value);
                    matrix[dy as usize][dx as usize].value = value + 1;
                    if !matrix[dy as usize][dx as usize].flashed_this_iteration && matrix[dy as usize][dx as usize].value > 9{
                        flash_substep(matrix, dy as usize, dx as usize, flashes);
                    }
                }
            }
        }
    }

}

fn simulate_steps(matrix: &mut Vec<Vec<Octopus>>, days: i32)->i32{
    let mut sum = 0;
    let matrix_amount_of_elements = matrix.len()*matrix[0].len();
    for i in 0..days{
        let flashes = simulate_one_step(matrix);
        sum = sum + flashes;
        // if i > 190 && i < 195{
        //     println!("Sum on i {} : {}", i, sum);
        // }
        if flashes == matrix_amount_of_elements  as i32{
            println!("all flashes on day: {}",i+1);
        }
    }
    sum
}

fn get_matrix_from_string(input: &String) -> Vec<Vec<Octopus>>{
    let mut res: Vec<Vec<Octopus>> = Vec::new();

    let rows = input.lines();
    for row in rows{
        let mut v: Vec<Octopus> = Vec::new();
        let c_arr =row.chars();
        for n in c_arr{
            let val = n.to_digit(10).unwrap() as i32;
            v.push(Octopus{flashed_this_iteration: false, value: val});
        }

        res.push(v);
    }
    res
}

fn read_input(input: &str)-> String{
    let contents = fs::read_to_string(input).expect("Something went wrong with reading the file.");
    contents.to_string()
}

struct Octopus{
    flashed_this_iteration: bool,
    value: i32,
}