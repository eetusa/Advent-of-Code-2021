use std::fs;
// use ansi_term::Style;
// use ansi_term::Colour::{Red};
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

    let lines = parse_lines_from_input(&v);

    let width = 999;
    let height = 999;
    let treshhold = 2;

    let mut board = create_board(width, height);
   
    for line in lines{
        set_line_on_board(&mut board, &line);
    }
  //  print_board(&board);
    let hotspots = get_board_point_amount_equal_or_over_treshhold(&board, treshhold);
    println!("Points over {}: {}", treshhold, hotspots);

}

fn get_board_point_amount_equal_or_over_treshhold(board: &Vec<Vec<i32>>, treshhold: i32) -> i32{
    
    let mut amount_over_treshhold = 0;
    for row in board{
        for value in row{
            if value >= &treshhold{
                amount_over_treshhold = amount_over_treshhold + 1;
            }
        }
    }
    amount_over_treshhold
}

fn set_line_on_board(board: &mut Vec<Vec<i32>>, line: &Line){
 //   let points_in_line = get_all_points_in_line_horizontal_vertical_only(line);
    let points_in_line = get_all_points_in_line(line);
    for point in points_in_line{
        let x = point.x;
        let y = point.y;
        let value = board[y][x];

        board[y][x] = value + 1;
    }
}

fn get_all_points_in_line(line: &Line) -> Vec<Point>{
    let mut all_points: Vec<Point> = Vec::new();
    let p1 = &line.p1;
    let p2 = &line.p2;

    if p1.x == p2.x{
        let x = p1.x;
        if p1.y < p2.y{
            for i in p1.y..p2.y+1{
                all_points.push(Point{x: x, y: i});
            }
        } else{
            for i in p2.y..p1.y+1{
                all_points.push(Point{x: x, y: i});
            }
        }
    } else if p1.y == p2.y{
        let y = p1.y;
        if p1.x < p2.x{
            for i in p1.x..p2.x+1{
                all_points.push(Point{x: i, y: y});
            }
        } else{
            for i in p2.x..p1.x+1{
                all_points.push(Point{x: i, y: y});
            }
        }
    } else{
        let k = (p2.x as i32-p1.x as i32)/(p2.y as i32-p1.y as i32);
      //  println!("k: {}", k);
        if p1.x < p2.x{
            let mut counter = 0;
            for x in p1.x..p2.x+1{
                let value = p1.y as i32 +k*counter;
                let value: usize = value as usize;
                all_points.push(Point{x: x, y: value});
                counter = counter + 1;
            }
        } else{
            let mut counter = 0;
            for x in p2.x..p1.x+1{
                let value = p2.y as i32 + k*counter;
                let value: usize = value as usize;
                all_points.push(Point{x: x, y: value});
                counter = counter + 1;
            }
        }
    }





    all_points
}

fn get_all_points_in_line_horizontal_vertical_only(line: &Line) -> Vec<Point>{
    let mut all_points: Vec<Point> = Vec::new();
    let p1 = &line.p1;
    let p2 = &line.p2;

    if p1.x == p2.x{
        let x = p1.x;
        if p1.y < p2.y{
            for i in p1.y..p2.y+1{
                all_points.push(Point{x: x, y: i});
            }
        } else{
            for i in p2.y..p1.y+1{
                all_points.push(Point{x: x, y: i});
            }
        }
    } else if p1.y == p2.y{
        let y = p1.y;
        if p1.x < p2.x{
            for i in p1.x..p2.x+1{
                all_points.push(Point{x: i, y: y});
            }
        } else{
            for i in p2.x..p1.x+1{
                all_points.push(Point{x: i, y: y});
            }
        }
    }

    all_points
}

fn print_board(board: &Vec<Vec<i32>>){
    println!();
    for row in board{
        for value in row{
            if *value == 0{
                print!(".");
            } else{
                print!("{}", value);
            }
            
        }
        println!(); 
    }
}

fn create_board(width: i32, height: i32) -> Vec<Vec<i32>>{
    let mut board = vec![vec![0; width.try_into().unwrap()]; height.try_into().unwrap()];

    board
}

fn parse_lines_from_input(lines: &Vec<&str>) -> Vec<Line>{
    let mut result: Vec<Line> = Vec::new();

    for row in lines{
        let row_string: String = row.to_string();
        let mut split = row_string.split_whitespace();
        
       // let op = split.nth(n)
        let mut first_split = split.nth(0).unwrap().split(",");
        let x1 = first_split.nth(0).unwrap();
        let y1 = first_split.nth(0).unwrap();

        let mut second_split = split.nth(1).unwrap().split(",");
        let x2 = second_split.nth(0).unwrap();
        let y2 = second_split.nth(0).unwrap();

        let new_line = Line{
            p1: Point{x: x1.parse().unwrap(), y: y1.parse().unwrap()},
            p2: Point{x: x2.parse().unwrap(), y: y2.parse().unwrap()},
        };
        
        result.push(new_line);
    }

    result
}

struct Line{
    p1: Point,
    p2: Point,
}

struct Point{
    x: usize,
    y: usize,
}

// fn print_matrix(matrix: &Vec<Vec<(i32, bool)>>){
//     println!();
//     for row in matrix{
//         for tuple in row{
//             let value = tuple.1;
//             let text = tuple.0.to_string();
//             if value{
//                 print!("{} ", Style::new().fg(Red).bold().paint(text));
//             } else{
//                 print!("{} ", text);
//             }
//           //  print!("{} ", tuple.0);
//         }
//         println!();
//     }
// }

fn read_input(input: &str) -> String{
    let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
    
    contents.to_string()
}