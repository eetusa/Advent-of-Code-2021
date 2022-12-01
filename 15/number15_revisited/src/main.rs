use std::{collections::{HashMap, BinaryHeap}, cmp::Reverse};

static DATA: &str = include_str!("input.txt");
fn main() {
  //  println!("Part a: {}", part_a(DATA));
    println!("part a: {}", part_a(DATA));
    println!("part b: {}", part_b(DATA));

    // let val = 7;
    // for i in 0..130{
    //     let mut new_val = val + i;
    //     while new_val > 9{
    //         new_val = new_val - 9;
    //     }

    //     println!("val: {}", new_val);
    // }
}


fn part_a(data: &'static str) -> u32{
    let grid = parse(data);
    best_risk(&grid)
}

fn part_b(data: &'static str) -> u32{
    let super_grid = get_super_grid(data);
    best_risk(&super_grid)
}

fn get_super_grid(data: &'static str) -> HashMap<(i16,i16), u32>{
    let mut result = HashMap::new();
    let mut temp = Vec::new();

    for line in data.lines(){
        temp.push(line);
    }

    let width = temp.len();
    let height = temp[0].len();

    for (y, line) in data.lines().enumerate(){
        for (x, c) in line.chars().enumerate(){
            let value = c.to_digit(10).unwrap();
            for i in 0..5{
                for j in 0..5{
                    let mut new_val = value + i as u32 + j as u32;
                    let new_x = (x + j*width) as i16;
                    let new_y = (y + i*height) as i16;
                    while new_val > 9{
                        new_val = new_val - 9;
                    }
                    result.insert((new_x, new_y), new_val as u32);
                  //  println!("({},{}), {}", new_x, new_y, new_val);
                }
            }
        }
    }

    result
}

fn best_risk(grid: &HashMap<(i16, i16), u32>) -> u32{
    let mut best_known = HashMap::new();
    let mut queue = BinaryHeap::from([(Reverse(0), 0, 0)]);
    while let Some( (Reverse(total_risk), x, y) ) = queue.pop() {
        let best_known_risk = best_known.entry((x,y)).or_insert(u32::MAX);
        if total_risk < *best_known_risk{
            *best_known_risk = total_risk;
            for (dx, dy) in [(1,0),(-1,0),(0,1), (0,-1)]{
                let (x,y) = (x+dx, y+dy);
                if let Some(risk) = grid.get(&(x,y)){
                    queue.push((Reverse(risk+total_risk), x, y));
                }
            }
        }
    }
    best_known[best_known.keys().max().unwrap()]
}

fn parse(data: &'static str) -> HashMap<(i16,i16), u32>{
    data.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
            .enumerate()
            .map(move |(x,c)|{
                ((x as i16, y as i16), c.to_digit(10).unwrap())
            })
        })
        .collect()
}

#[cfg(test)]
mod tests{
    use super::*;
    const SAMPLE_DATA: &str = include_str!("test_input.txt");

    #[test]
    fn test_a(){
        assert_eq!(part_a(SAMPLE_DATA), 40);
    }

    #[test]
    fn test_b(){
        assert_eq!(part_b(SAMPLE_DATA), 315);
    }
    

}