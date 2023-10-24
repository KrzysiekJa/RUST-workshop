use std::collections::HashMap;


pub fn decrypt(text: String, shift_count: u8, shift_rigth: bool) -> String
{
    let mut decrypted_text = "".to_string();

    for c in text.chars(){
        if char::is_whitespace(c){
            decrypted_text.push(c);
            continue;
        }
        let char_as_number: u8 = c as u8;
        let shifted_char: char = if shift_rigth { char_as_number + shift_count } else { char_as_number - shift_count } as char;
        decrypted_text.push(shifted_char);
    }
    return decrypted_text;
}


pub fn sum_number_sequence(n: i32) -> i32
{
    let mut final_sum: i32 = 0;

    for i in 1..n+1
    {
        let mut str: String = String::new();
        for _j in 0..i
        {
            str.push_str( &i.to_string() );
        }
        final_sum += str.parse::<i32>().unwrap();
    }

    return final_sum;
}

pub fn factor(n: i32) -> i32
{
    let mut res: i32 = 1;

    for i in 1..n+1{
        res *= i;
    }
    return res;
}

pub fn multiplication_seq(n: i32) -> i32
{
    let mut res: i32 = 1;

    for i in 1..n+1{
        res *= 2*i - 1;
    }
    return res;
}

pub fn division_seq(n: i32) -> f32
{
    let mut res: f32 = 1.0;

    for i in 1..n+1{
        res *= 1.0 / ( (i * (i+1)) as f32);
    }
    return res;
}

pub fn inverse_squares_seq(n: i32) -> f32
{
    let mut res: f32 = 1.0;

    for i in 1..n+1{
        res *= 1.0 / ( i32::pow(i, 2) as f32);
    }
    return res;
}

pub fn inverse_cubes_seq(n: i32) -> f32
{
    let mut res: f32 = 1.0;

    for i in 1..n+1{
        res *= 1.0 / ( i32::pow(i, 3) as f32);
    }
    return res;
}

pub fn find_smallest_divisible_number_by_7(limit: i32) -> i32
{
    let mut i: i32 = 7;

    while i <= limit {
        if i % 4 == 1 && i % 5 == 1 && i % 6 == 1 {
            break;
        }
        i = i + 7;
    }
    return i;
}

pub fn count_numbers_divisible_by_7(limit: i32) -> i32
{
    let mut i: i32 = 7;
    let mut count: i32 = 0;

    while i <= limit {
        if i % 4 == 1 && i % 5 == 1 && i % 6 == 1 {
            count = count + 1;
        }
        i = i + 7;
    }
    return count;
}

pub fn inverse_calculation_merchant(n: i32) -> i32
{
    let mut res: i32 = 0;

    for _i in 1..n+1{
        res += 12;
        res /= 2;
    }
    return res;
}

pub fn generate_and_print_fibonacci(n: u32)
{
    let mut a_0: u128 = 0;
    let mut a_1: u128 = 1;
    let mut tmp: u128;

    for _i in 1..n+1{
        print!("{} ", a_0);
        tmp = a_1;
        a_1 += a_0;
        a_0 = tmp;
    }
}

pub fn get_multiplication_table()
{
    for i in 0..11
    {
        for j in 0..11
        {
            print!("{} ", i * j);
        }
        print!("\n");
    }
}

pub fn get_factorial(num: u128) -> u128
{
    return (1..=num).product()
}

pub fn read_number(num: i32) -> String
{
    let mut final_str = String::new();
    let mut tmp_str = "";
    
    for i_char in num.to_string().chars() {
        match i_char {
            '0' => tmp_str = "zero",
            '1' => tmp_str = "one",
            '2' => tmp_str = "two",
            '3' => tmp_str = "three",
            '4' => tmp_str = "four",
            '5' => tmp_str = "five",
            '6' => tmp_str = "six",
            '7' => tmp_str = "seven",
            '8' => tmp_str = "eight",
            '9' => tmp_str = "nine",
            '-' => tmp_str = "minus",
            _ => println!("Invalid character: {}", i_char),
        }
        final_str.push_str(tmp_str);
        final_str.push_str(" ");
    }
    return final_str;
}

pub fn get_info_from_people_list( people_list: Vec<&str> )
{
    let mut people_list: Vec<String> = people_list.iter().map(|&s| s.to_lowercase()).collect();

    let min_name = people_list.iter().fold(people_list.get(0), |acc, item| {
        if item.len() < acc.unwrap().len() { Some(&item) } else { acc }
    });
    let max_name = people_list.iter().fold(people_list.get(0), |acc, item| {
        if item.len() > acc.unwrap().len() { Some(&item) } else { acc }
    });
    println!("Names: min: {}, max: {}", min_name.unwrap(), max_name.unwrap());

    people_list.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("Sorted: {:?}", people_list);
}

pub fn make_christmas_tree(n: u32)
{
    for i in 0..n
    {
        for _j in i..(n+1)*2
        {
            print!(" ");
        }
        for _j in 0..(i+1)*2 - 1
        {
            print!("*");
        }
        print!("\n");
    }
}

pub fn calculate_cos(x_val: f32) -> f32
{
    let mut cos_val: f32 = 0.0;
    let mut c_i: f32 = 1.0; // c_0 = 1
    let n_iter: i32 = 15;

    for i in 1..n_iter+1 // starting with c_1
    {
        cos_val += c_i;
        c_i = -c_i * (f32::powf(x_val, 2.0) / ( (2*i * (2*i-1)) as f32));
    }
    return cos_val;
}

pub fn convert_numbers_to_systems(decimal_val: u32) -> HashMap<&'static str,String>
{
    let mut res_map: HashMap<&str, String> = HashMap::new();
    res_map.insert("decimal", decimal_val.to_string());
    res_map.insert("binary", format!("{:b}", decimal_val));
    res_map.insert("octal", format!("{:o}", decimal_val));
    res_map.insert("hexadecimal", format!("{:x}", decimal_val));
    return res_map;
}
