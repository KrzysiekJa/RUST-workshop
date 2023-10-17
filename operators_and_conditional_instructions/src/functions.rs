use std::collections::HashMap;
use chrono::Datelike;

pub fn heaviside(x: f32) -> u8
{
    return ( x >= 0.0 ) as u8;
}

pub fn absolute_val(mut x: f32) -> f32
{
    if x < 0.0 {
        x = -x;
    }
    return x;
}

pub fn sng(x: f32) -> i8
{
    if x >= 0.0 {
        return 1;
    }
    return -1;
}

pub fn floor(x: f32) -> f32
{
    return x as i32 as f32;
}

pub fn mantissa(x: f32) -> f32
{
    return x - floor(x);
}

pub fn is_trangle(a: f32, b: f32, c: f32) -> ( f32, f32 )
{
    if !( a + b > c && a + c > b && b + c > a ) {
        return ( 0.0, 0.0);
    }
    let p: f32 = ( a+b+c ) / 2.0;
    let area: f32 = ( p*( p-a )*( p-b)*( p-c ) ).sqrt();    

    return ( 2.0*p, area );
}

pub fn get_sgn_triangle(x: (f32, f32), y: (f32, f32), z: (f32, f32)) -> f32
{
    return (x.0 - z.0) * (y.1 - z.1) - (y.0 - z.0) * (x.1 - z.1);
}

pub fn is_in_trangle(p: (f32, f32) ) -> bool
{
    let a = (-5.0, 5.0);
    let b = (5.0, -5.0);
    let c = (-5.0, -5.0);

    let d1 = get_sgn_triangle(p, a, b);
    let d2 = get_sgn_triangle(p, b, c);
    let d3 = get_sgn_triangle(p, c, a);

    let has_negative = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
    let has_positive = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);

    return !(has_negative && has_positive);
}

pub fn get_number_of_quadrant_of_cartesian(x: f32, y: f32) -> i8
{
    if x >= 0.0 && y >= 0.0{
        return 1;
    }
    if x < 0.0 && y >= 0.0{
        return 2;
    }
    if x < 0.0 && y < 0.0{
        return 3;
    }
    if x >= 0.0 && y < 0.0{
        return 4;
    }
    return -1;
}

pub fn split_sentence_into_words(sentence: &str) -> Vec<String>
{
    let words: Vec<&str> = sentence.split_whitespace().collect();
    let words_vec: Vec<String> = words.iter().map(|&s| s.to_string()).collect();
    return words_vec;
    // alternatively:
    // sentence
    //     .split_whitespace()
    //     .map(String::from)
    //     .collect()
}

pub fn decode_PESEL( pesel_code: String ) -> Option< HashMap<String, String> >
{
    if pesel_code.len() != 11 && !pesel_code.parse::<i32>().is_ok() {
        return None;
    }

    let mut pesel_data: HashMap<String, String> = Default::default();
    let sex: String;
    let sex_digit: char = pesel_code.chars().nth(9).unwrap();
    
    if sex_digit as u32 % 2 == 1 {
        sex = "male".to_string();
    } else {
        sex = "female".to_string();
    }
    pesel_data.insert( "sex".to_string(), sex );

    let century: u32 = pesel_code.chars().nth(0).unwrap().to_digit(10).unwrap() * 10 + pesel_code.chars().nth(1).unwrap().to_digit(10).unwrap();
    let month_digit = pesel_code[2..4].parse::<u32>().unwrap();
    let months = vec![
        "January", "February", "March", "April", "May", "June", "July", "August",
        "September", "October", "November", "December",
    ];
    let current_year = chrono::Utc::now().year();

    if month_digit as i32 <= 12 {
        let birth_year = 1900 + century as i32;
        pesel_data.insert( "month".to_string(), months[month_digit as usize - 1].to_string() );    
        pesel_data.insert( "age".to_string(), (current_year - birth_year).to_string() );
    } else {
        let birth_year = 2000 + century as i32;
        pesel_data.insert( "month".to_string(), months[month_digit as usize - 21].to_string() );    
        pesel_data.insert( "age".to_string(), (current_year - birth_year).to_string() );
    }

    return Some( pesel_data );
}
