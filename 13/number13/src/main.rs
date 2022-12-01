use std::fs;
fn main() {
    let input = read_input("input.txt");
    first(&input);
}

fn first(input: &String){

    let mut board = get_board(input);
    let mut folds = get_folds(input);
    let amount_of_folds = folds.len();
   
    fold_board(&mut board, &mut folds, amount_of_folds as i32);

    draw_board(&board);

    let amount_of_dots = get_amount_of_dots(&board);
    println!("Dots after {} folds: {}", amount_of_folds, amount_of_dots);
    
}

fn get_amount_of_dots(board: &Vec<Vec<i32>>) -> i32{
    let mut sum = 0;
    
    for row in board{
        for value in row{
            if *value > 0{
                sum = sum +1;
            }
        }
    }

    sum
}

fn fold_board(board: &mut Vec<Vec<i32>>, folds: &mut Vec<Fold>, amount_of_folds: i32){
    for i in 0..amount_of_folds{
        fold_once(board, &folds[i as usize]);
    }
}

fn fold_once(board: &mut Vec<Vec<i32>>, fold: &Fold){
    let axis = fold.axis;
    let fold_value = fold.value;

    if axis == 'x'{
     //   println!("Fold x = {}", fold_value);

        for i in 0..board.len(){
            for j in fold_value..board[0].len() as i32{
                let is_point = board[i as usize][j as usize] != 0;
                if is_point{

                    let distance = j - fold_value;
                   // println!("x: {}, y: {}, x-distance: {}", j, i, distance);
                    let target_x = fold_value - distance;

                    board[i as usize][target_x as usize] = board[i as usize][target_x as usize] + 1;
                }
            }
        }

        let col_len = board[0].len();
        let extra_cols = col_len-fold_value as usize;
        for i in 0..board.len(){
            for _ in 0..extra_cols{
                board[i].pop();
            }
        }


    }  else {
    //    println!("Fold y = {}", fold_value);

        for i in fold_value..board.len() as i32{
            for j in 0..board[0].len(){
                let is_point = board[i as usize][j as usize] != 0;
                if is_point{
                    let distance = i - fold_value;
                    let target_y = fold_value  - distance;

                    board[target_y as usize][j as usize] = board[target_y as usize][j as usize] + 1;
                }
            }
        }
        let row_len = board.len();
        let extra_rows = row_len-fold_value as usize;
        for _ in 0..extra_rows{
            board.pop();
        }
    }
}



fn get_folds(input: &String) -> Vec<Fold>{

    let mut folds = Vec::new();
    let lines = input.lines();
    let mut parse_line = true;
    for line in lines{
        if line.eq(&"".to_string()){
            parse_line = false;
        }
        if parse_line{

        } else {
            if !line.eq(&"".to_string()){
                let mut split = line.split('=');
                let ax = split.nth(0).unwrap();
                let val = split.nth(0).unwrap().parse().unwrap();
                if ax.contains('x'){
                    folds.push(Fold{axis: 'x', value: val})
                } else {
                    folds.push(Fold{axis: 'y', value: val})
                }
            }
        }

    }  

    folds
}

fn get_board(input: &String) -> Vec<Vec<i32>>{
    let mut board = Vec::new();
    let mut x_max = 0;
    let mut y_max = 0;

    let lines = input.lines();
    let mut parse_line = true;
    for line in lines{
        if line.eq(&"".to_string()){
            parse_line = false;
        }
        if parse_line{
            let mut split = line.split(',');
            let x = split.nth(0).unwrap().parse().unwrap();
            let y = split.nth(0).unwrap().parse().unwrap();
            x_max = std::cmp::max(x_max, x);
            y_max = std::cmp::max(y_max, y);
        }
    }

    for i in 0..y_max+1{
        board.push(Vec::new());
        for _ in 0..x_max+1{
            board[i].push(0);
        }
    }

    let lines = input.lines();
    let mut parse_line = true;
    for line in lines{
        if line.eq(&"".to_string()){
            parse_line = false;
        }
        if parse_line{
            let mut split = line.split(',');
            let x: i32 = split.nth(0).unwrap().parse().unwrap();
            let y: i32 = split.nth(0).unwrap().parse().unwrap();
            board[y as usize][x as usize] = 1;
        }
    }

    board
}

fn draw_board(board: &Vec<Vec<i32>>){
    for i in 0..board.len(){
        for j in 0..board[i].len(){
            let val = board[i][j];
            if val == 0{
                print!(".");
            } else{
                print!("#");
            }
        }
        println!();
    }
    println!();
}

struct Fold{
    axis: char,
    value: i32,
}

fn read_input(input: &str) -> String{
    let contents = fs::read_to_string(input)
            .expect("Something went wrong reading the file");
        
    contents.to_string()
}