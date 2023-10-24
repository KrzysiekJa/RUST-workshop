use std::f32::consts::PI;
use std::collections::HashMap;
mod utils;

fn main() {

    // exercise 1
    println!(">> 1");
    let coded_string = String::from("Rmgi$ he}% ");
    for i in 1..11
    {
        let out_string = utils::decrypt(coded_string.clone(), i, true);
        println!("{} shifted:     {}", i, out_string);
        let out_string = utils::decrypt(coded_string.clone(), i, false);
        println!("{} not shifted: {}", i, out_string);
    }

    // exercise 2
    println!(">> 2");
    println!("for n=2: {}", utils::sum_number_sequence(2) );
    println!("for n=5: {}", utils::sum_number_sequence(5) );
    println!("for n=9: {}", utils::sum_number_sequence(9) );

    // exercise 3
    println!(">> 3");
    println!( "factor!");
    println!( "for n=3: {}", utils::factor(3) );
    println!( "for n=5: {}", utils::factor(5) );
    println!( "for n=7: {}", utils::factor(7) );
    println!( "sequence 2*n-1");
    println!( "for n=3: {}", utils::multiplication_seq(3) );
    println!( "for n=5: {}", utils::multiplication_seq(5) );
    println!( "for n=7: {}", utils::multiplication_seq(7) );
    println!( "sequence 1/(n(n+1))");
    println!( "for n=3: {}", utils::division_seq(3) );
    println!( "for n=5: {}", utils::division_seq(5) );
    println!( "for n=7: {}", utils::division_seq(7) );
    println!( "1/(n^2)");
    println!( "for n=3: {}", utils::inverse_squares_seq(3) );
    println!( "for n=5: {}", utils::inverse_squares_seq(5) );
    println!( "for n=7: {}", utils::inverse_squares_seq(7) );
    println!( "1/(n^3)");
    println!( "for n=3: {}", utils::inverse_cubes_seq(3) );
    println!( "for n=5: {}", utils::inverse_cubes_seq(5) );
    println!( "for n=7: {}", utils::inverse_cubes_seq(7) );

    // exercise 4
    println!(">> 4");
    let smallest_divisible_number_by_7 = utils::find_smallest_divisible_number_by_7( 1000 );
    println!( "The smallest divisible number 7 which, when divided by 2, 3, 4, 5, 6, gives the remainder r = 1:" );
    println!( "{}", smallest_divisible_number_by_7 );

    // exercise 5
    println!(">> 5");
    let counted_numbers_divisible_by_7_lim_100 = utils::count_numbers_divisible_by_7( 100 );
    println!( "Counted numbers divisible 7 which, when divided by 2, 3, 4, 5, 6, gives the remainder r = 1:" );
    println!( "limit 100: {}", counted_numbers_divisible_by_7_lim_100 );
    let counted_numbers_divisible_by_7_lim_2000 = utils::count_numbers_divisible_by_7( 2000 );
    println!( "Counted numbers divisible 7 which, when divided by 2, 3, 4, 5, 6, gives the remainder r = 1:" );
    println!( "limit 2000: {}", counted_numbers_divisible_by_7_lim_2000 );

    // exercise 6
    println!(">> 6");
    let mut merchant_capital = utils::inverse_calculation_merchant(2);
    println!( "Merchant original capital for Venecia-Florencia: {}", merchant_capital );
    merchant_capital = utils::inverse_calculation_merchant(3);
    println!( "Merchant original capital for Venecia-Florencia-Pisa: {}", merchant_capital );

    // exercise 7
    println!(">> 7");
    utils::generate_and_print_fibonacci( 100 );
    println!( "" );

    // exercise 8
    utils::get_multiplication_table();

    // exercise 9
    println!(">> 9");
    let factorial_15 = utils::get_factorial( 15 );
    let factorial_str = factorial_15.to_string();
    println!( "factorial 15 starts with: {}", factorial_str.get(0..3).unwrap() );

    // exercise 10
    println!(">> 10");
    let mut number2read = -321;
    let mut number_string = utils::read_number(number2read);
    println!( "{} is: {}", number2read, number_string );
    number2read = 0;
    number_string = utils::read_number(number2read);
    println!( "{} is: {}", number2read, number_string );

    // exercise 11
    println!(">> 11");
    let people_list: Vec<&str> = vec!["Eric", "Mandy", "Jen", "Josh", "David", "Matt", "Danielle", "Willie"];
    println!("Sorted: {:?}", people_list);
    utils::get_info_from_people_list( people_list );

    // exercise 12
    println!(">> 12");
    utils::make_christmas_tree(5);

    // exercise 13
    println!(">> 13");
    let mut x_cos = 2.0 * PI;
    let mut y_cos = utils::calculate_cos(x_cos);
    println!("cos({}) = {}", x_cos, y_cos);
    x_cos = 0.5 * PI;
    y_cos = utils::calculate_cos(x_cos);
    println!("cos({}) = {}", x_cos, y_cos);
    x_cos = 0.5;
    y_cos = utils::calculate_cos(x_cos);
    println!("cos({}) = {}", x_cos, y_cos);

    // exercise 14
    println!(">> 14");
    let res_map: HashMap<&str, String> = utils::convert_numbers_to_systems(15);
    println!("{} -> b:{} o:{} h:{}", res_map.get(&"decimal").unwrap(), res_map.get(&"binary").unwrap(), 
            res_map.get(&"octal").unwrap(),res_map.get(&"hexadecimal").unwrap());

}
