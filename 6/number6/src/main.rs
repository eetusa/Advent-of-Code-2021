use std::fs;
fn main() {
    let input = read_input("input.txt");
    second(&input);
}

fn second(input: &String){
    let mut fishes_input: Vec<i32> = input.split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let amount_of_days = 256;
    print_fishes(&fishes_input, 0);

    let mut fishes: [i64;9] = [0,0,0,0,0,0,0,0,0];

    for value in fishes_input{
        fishes[value as usize] = fishes[value as usize] + 1;
    }

    simulate_fishes_optimized(&mut fishes, amount_of_days);

    let mut sum = 0;
    for i in 0..fishes.len(){
        sum = sum + fishes[i];
    }

    println!("Summa: {}", sum);

    
}

fn simulate_fishes_optimized(fishes: &mut [i64;9], days: i32){
    for i in 0..days{
        let zero_index = fishes[0];
        fishes[0] = fishes[1];
        fishes[1] = fishes[2];
        fishes[2] = fishes[3];
        fishes[3] = fishes[4];
        fishes[4] = fishes[5];
        fishes[5] = fishes[6];
        fishes[6] = zero_index + fishes[7];
        fishes[7] = fishes[8];
        fishes[8] = zero_index;
    }
}

fn first(input: &String){
    let mut v: Vec<&str> = Vec::new();
    let lines = input.lines();
    for line in lines{
        v.push(line);
    }

  //  let mut all_fishes: Vec<i32> = v[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect();
   // print_fishes(&all_fishes, 0);
   
   let mut all_fishes: Vec<i32> = Vec::with_capacity(u32::MAX as usize);
   let split = v[0].split(",");
   for value in split{
       let value_nmbr: i32 = value.parse().unwrap();
       all_fishes.push(value_nmbr);
   }


    let amount_of_days = 80;

    for i in 0..amount_of_days{
        simulate_fishes(&mut all_fishes, 1);
      //  print_fishes(&all_fishes, i+1);
    }
    
    println!("Total amount of fishes: {}", all_fishes.len());
    
}

fn simulate_fishes(fishes: &mut Vec<i32>, days: i32){
    for i in 0..days{
        let amount_of_fishes_at_day_begin = fishes.len();
        for j in 0..amount_of_fishes_at_day_begin{
            let value = fishes[j];
            if (value == 0){
                fishes[j] = 6;
                fishes.push(8);
            } else if (value > 0){
                fishes[j] = value - 1;
            }
        }
    }
}

fn print_fishes(fishes: &Vec<i32>, days: i32){
    if (days < 10){
        if (days == 0){
            print!("Initial state: ");
        } else if (days == 1){
            print!("After  {} day:  ", days);
        }else{
            print!("After  {} days: ", days);
        }
        
    }else{
        print!("After {} days: ", days);
    }
    for fish in fishes{
        print!("{},",fish);
    }
    println!();
}

fn read_input(input: &str)-> String{
    let contents = fs::read_to_string(input).expect("Something went wrong with reading the file.");
    contents.to_string()
}