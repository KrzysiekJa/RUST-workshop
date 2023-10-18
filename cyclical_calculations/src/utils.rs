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
        for j in 0..i
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
