const DATA: &str = include_str!("input.txt");

fn main() {
    println!("Part a: {}", part_a(DATA));
}

fn part_a(data: &'static str) -> u32{
    println!("Bit: {}", convert_to_binary_from_hex(&"620080001611562C8802118E34"));
    0
}

fn get_version_number_sum_n_subpackets(sub_data: &'static str, n: u32) -> u32{
    let mut v_n_sum = 0;
    let current_packet_start_index = 0;
    for i in 0..n{

        let slice = &sub_data[current_packet_start_index..];
        let p_v = get_packet_version(slice);
        let t_id = get_type_id(slice);

        if t_id != 4{
            let length_type_is_zero = is_length_type_zero(sub_data);
            if length_type_is_zero{
    
            }else {
    
            }
        } else {
           // let val = get_literal_value(sub_data);
            v_n_sum = v_n_sum + p_v;
        }
    
    }

    v_n_sum
}

// fn parse_through_binary(sub_data: &'static str) -> u32{
//     let mut sub_index = 0;
//     for c in sub_data.chars(){
//         if sub_index < 3{
//             version_number.push(c);
//             continue;
//         } 
        
//     }
//     0
// }


fn parse_through_binary(sub_data: &str) -> u32{
    println!("bin: {}", sub_data);
    let mut literal_values = Vec::new();
    let mut sum = 0;
    
    
    let mut version_number = String::new();
    let mut type_id = String::new();
    let mut literal_temp_counter = -1;
    let mut literal_last_call = false;
    let mut literal_temp_string = String::new();
    let mut length_type_id = "-1".to_string();

    let mut length_type_id_0_bit_counter = 0;
    let mut length_type_id_0_temp = String::new();
    let mut length_type_id_bit_count

    let mut type_id_decimal = 9999999;
    let mut version_decimal = 9999999;
    let mut sub_index = -1;


    for c in sub_data.chars(){

        sub_index = sub_index + 1;
        println!("Sub_index: {}", sub_index);

        if sub_index < 3{
            version_number.push(c);
            continue;
        } 

        if sub_index == 3{
            version_decimal = convert_bit_to_decimal(&version_number);
            sum = sum + version_decimal;
            version_number.clear();
        }

        if sub_index < 6{
            type_id.push(c);
            continue;
        }

        if sub_index == 6{
            type_id_decimal = convert_bit_to_decimal(&type_id);
            println!("version_decimal: {}, type_id_decimal: {}, ", version_decimal, type_id_decimal);
            type_id.clear();
        }


        if type_id_decimal == 4{
            literal_temp_counter = literal_temp_counter + 1;
            println!("temp_counter: {}", literal_temp_counter);
            if literal_temp_counter % 5 == 0{
                if c == '0'{
                    literal_last_call = true;
                    println!("Last call");
                }
                continue;
            } else {
                literal_temp_string.push(c);
                if literal_last_call{
                    if literal_temp_string.len()%4==0{
                        //push literal value & reset
                        literal_values.push(convert_bit_to_decimal(&literal_temp_string));
                        println!("bits: {}", literal_temp_string);
                        literal_temp_counter = 0;
                        literal_last_call = false;
                        literal_temp_string.clear();
                        sub_index = -1;
                        length_type_id = "-1".to_string();
                    }
                }
            }

        } else { // type_id != 4
            if length_type_id == "-1"{
                length_type_id = c.to_string();
            }
            if length_type_id == "0".to_string(){
                if length_type_id_0_bit_counter < 15{
                    length_type_id_0_temp.push(c);
                    continue;
                }
                length_type_id_0_bit_counter = length_type_id_0_bit_counter + 1;
                if length_type_id_0_bit_counter == 15{

                }
            } else {

            }
        }
    }
    for v in &literal_values{
        println!("Literal: {}", v);
    }
    sum
}

fn get_literal_value(binary_data: &str) -> u32{
    let slice = &binary_data[6..];

    let mut counter = 0;
    let mut bit_string = String::new();
    let mut end_found = false;
    for c in slice.chars(){
        counter = counter + 1;
        let value = c.to_digit(10).unwrap();
        if counter < 6{
            if counter == 1{
                if value == 0{
                    end_found = true;
                }
            } else {
                bit_string.push(c);
            }
            if counter == 5{
                counter = 0;
                if end_found{
                    break;
                }
            }
        }
    }

    convert_bit_to_decimal(&bit_string)
}

fn get_type_id(binary: &str) -> u32{
    let slice = &binary[3..6];
    convert_bit_to_decimal(slice)
}

fn is_length_type_zero(binary: &str) -> bool{
    let slice = &binary[6..7];
    slice.eq("0")
}

fn get_packet_version(binary: &str) -> u32{
    let slice = &binary[..3];
    convert_bit_to_decimal(slice)
}

fn convert_bit_to_decimal(bit: &str) -> u32{
    let mut total = 0;
    let highest_power = bit.len()-1;
    for (index, c) in bit.chars().enumerate(){
        let power = highest_power-index;
        if c == '1'{
            total = total + 2i32.pow(power as u32);
        }
    }
    total as u32
}

fn to_binary(c: char) -> &'static str{
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn convert_to_binary_from_hex(hex_input: &'static str) -> String{
    hex_input.chars().map(to_binary).collect()
}

#[cfg(test)]
mod tests{
    use super::*;
    const SAMPLE_DATA: &str = "D2FE28";
    const SAMPLE_DATA_2: &str = "38006F45291200";

   // #[test]
    // fn part_a_1(){
    //     assert_eq!(30, part_a(SAMPLE_DATA_2));
    // }

    #[test]
    fn literal_value_test(){
        let bin = convert_to_binary_from_hex(SAMPLE_DATA);
        assert_eq!(6, parse_through_binary(&bin));
    }

    #[test]
    fn type_id_4(){
        let bin = convert_to_binary_from_hex(SAMPLE_DATA);
        assert_eq!(4, get_type_id(&bin));
    }

    #[test]
    fn packet_version_is_6(){
        let bin =convert_to_binary_from_hex(SAMPLE_DATA);
        assert_eq!(6, get_packet_version( &bin ));
    }

    #[test]
    fn zerozerzero_to_0(){
        assert_eq!("0000", to_binary('0'));
    }

    #[test]
    fn oneoneonezero_to_e(){
        assert_eq!("1110", to_binary('E'));
    }
    #[test]
    fn hex_to_binary_test(){
        assert_eq!("110100101111111000101000", convert_to_binary_from_hex(SAMPLE_DATA));
    }

    #[test]
    fn bit_to_decimal_1(){
        assert_eq!(1, convert_bit_to_decimal("1"));
    }

    #[test]
    fn bit_to_decimal_110(){
        assert_eq!(6, convert_bit_to_decimal("110"));
    }

    #[test]
    fn bit_to_decimal_1101(){
        assert_eq!(13, convert_bit_to_decimal("1101"));
    }
    #[test]
    fn bit_to_decimal_11011010(){
        assert_eq!(218, convert_bit_to_decimal("11011010"));
    }


}