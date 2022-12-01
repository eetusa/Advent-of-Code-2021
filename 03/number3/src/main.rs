use std::{fs, f32::RADIX};
fn main() {
    let input = read_input("input.txt");

 //   first(input);
    second(&input);
}

fn second(input: &String){
    let mut original: Vec<&str> = Vec::new();
    let lines = input.lines();
    for line in lines{
        original.push(line);
    }

    let mut oxygen_generator_rating: String = "".to_string();
    let mut co2_scrubber_rating: String = "".to_string();
    let mut index = 0;
    let mut accepted: Vec<&str> = Vec::clone(&original);


    while true{
        let mut one_half: Vec<&str> = Vec::new();
        let mut zero_half: Vec<&str> = Vec::new();
        for value in &accepted{
            let mut characters = value.chars();
            if characters.nth(index).unwrap() == '1'{
                one_half.push(value);
            } else {
                zero_half.push(value);
            }
        }

        if one_half.len() >= zero_half.len(){
            accepted = Vec::clone(&one_half);
        } else {
            accepted = Vec::clone(&zero_half);
        }
        index = index + 1;


        if accepted.len() == 1{
            oxygen_generator_rating = accepted[0].to_string();
            println!("OGR: {}, decimal: {}", oxygen_generator_rating, binary_to_decimal(&oxygen_generator_rating));
            break;
        }
    }

    let mut index = 0;
    let mut accepted: Vec<&str> = Vec::clone(&original);


    while true{
        let mut one_half: Vec<&str> = Vec::new();
        let mut zero_half: Vec<&str> = Vec::new();
        for value in &accepted{
            let mut characters = value.chars();
            if characters.nth(index).unwrap() == '1'{
                one_half.push(value);
            } else {
                zero_half.push(value);
            }
        }

        if one_half.len() < zero_half.len(){
            accepted = Vec::clone(&one_half);
        } else {
            accepted = Vec::clone(&zero_half);
        }
        index = index + 1;


        if accepted.len() == 1{
            co2_scrubber_rating = accepted[0].to_string();
            println!("CO2SR: {}, decimal: {}", co2_scrubber_rating, binary_to_decimal(&co2_scrubber_rating));
            break;
        }
    }
    println!("Multiplication: {}", binary_to_decimal(&oxygen_generator_rating) * binary_to_decimal(&co2_scrubber_rating));

}

fn first(input: String){

    let mut v: Vec<&str> = Vec::new();
    let lines = input.lines();
    for line in lines{
        v.push(line);
    }

    let mut gamma_rate: String = "".to_string();
    let mut epsilon_rate: String = "".to_string();

    let n = v.len();
    let treshhold = n/2;
    let row_length = v[0].len();

    let mut arr = vec![0; row_length];

    for i in 0..v.len(){
        let characters = v[i].chars();
        for (pos, char) in characters.enumerate(){
            let bit_value: u32 = char.to_digit(RADIX).unwrap();
            arr[pos] = arr[pos] + bit_value;
        }
    }

    for i in 0..arr.len(){
        if arr[i] > treshhold.try_into().unwrap(){
            gamma_rate = gamma_rate + "1";
            epsilon_rate = epsilon_rate + "0";
        } else{
            gamma_rate = gamma_rate + "0";
            epsilon_rate = epsilon_rate + "1";
        }
    }

    let gamma_rate_decimal: i32 = binary_to_decimal(&gamma_rate);
    let epsilon_rate_decimal: i32 = binary_to_decimal(&epsilon_rate);

    println!("N: {}", n);
    println!("Gamma_rate binary: {}, decimal: {}", gamma_rate, gamma_rate_decimal);
    println!("Epsilon rate binary: {}, decimal: {}", epsilon_rate, epsilon_rate_decimal);
    println!("Multiplied: {}", gamma_rate_decimal*epsilon_rate_decimal);
}

fn binary_to_decimal(input: &String) -> i32{
    let bin_len = input.len();
    let mut total_decimal = 0;
    
    for i in 0..bin_len{
        let power = bin_len-1-i;
        if input.chars().nth(i).unwrap() == '1'{
            total_decimal += i32::pow(2,power.try_into().unwrap());
        }
    }
    return total_decimal
}

fn read_input(input: &str) -> String{
    let contents = fs::read_to_string(input)
        .expect("Something went wrong reading the file");
    
    contents.to_string()
}