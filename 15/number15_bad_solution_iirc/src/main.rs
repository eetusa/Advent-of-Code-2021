use std::fs;
fn main() {
    let input = read_input("test_input.txt");
    first(&input);
}

fn first(input: &String){
    let mut matrix = construct_node_matrix(input);
    print_node_matrix(&matrix);
    let shortest_path = find_shortest_path(&mut matrix);
}

fn find_shortest_path(matrix: &mut Vec<Vec<Node>>){

    let mut shortest_path = ShortestPath{shortest_path: Vec::new()};
    let mut row = &mut matrix[0];
    let ref_node = &mut row[0];
    ref_node.previous_node_on_shortest = (3,5);
    println!("{}",&matrix[0][0].previous_node_on_shortest.0);
  //  let temp = Node{distance_to_this: ref_node.distance_to_this, shortest_distance: ref_node.distance_to_this, previous_node_on_shortest: ref_node.previous_node_on_shortest.as_ref()};
   // shortest_path.shortest_path.push(row[0]);
}

fn construct_node_matrix(input: &String) -> Vec<Vec<Node>>{
    let mut result = Vec::new();
    let lines = input.lines();
    for (index,line) in lines.enumerate(){
        let mut row: Vec<Node> = Vec::new();
        let chars = line.chars();
        for c in chars{
            let value = c.to_digit(10).unwrap() as i32;
            let node = Node{distance_to_this: value, shortest_distance: i32::MAX, previous_node_on_shortest: (-1, -1)};
            row.push(node);
        }
        result.push(row);
    }
    result
}

fn print_node_matrix(matrix: &Vec<Vec<Node>>){
    println!();
    for row in matrix{
        for node in row{
            print!("{}", node.distance_to_this);
        }
        println!();
    }
}
struct Node {
    distance_to_this: i32,
    shortest_distance: i32,
    previous_node_on_shortest: (i32, i32),
}
struct ShortestPath{
    shortest_path: Vec<Node>,
}



fn read_input(input: &str) -> String{
    let contents = fs::read_to_string(input)
            .expect("Something went wrong reading the file");
        
    contents.to_string()
}