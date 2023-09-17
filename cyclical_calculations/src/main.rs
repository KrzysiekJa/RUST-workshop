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


}
