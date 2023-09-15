use std::collections::HashMap;
mod exercises;

fn main() {

    // exercise 0
    println!(">> 0");
    exercises::exercise_types();
    
    // exercise 1
    println!(">> 1");
    let ball_results: [f32; 3] = exercises::get_throwed_ball_pos( 3.44, 9.81, [0.54, 0.1, 0.235] );
    println!("Throwed ball on Earth: {} {} {}", ball_results[0], ball_results[1], ball_results[2]);

    // exercise 2
    println!(">> 2");
    let ball_pos_vec: Vec<[f32; 3]> = exercises::get_throwed_ball_pos_planets( 
        3.44 , 
        vec![3.7, 8.8, 9.8, 3.7, 0.27, 24.7, 10.5, 9.0, 11.7], 
        [0.54, 0.1, 0.235] 
    );
    for pos_arr in ball_pos_vec.iter() {
        println!("Throwed ball : {} {} {}", pos_arr[0], pos_arr[1], pos_arr[2]);
    }

    // exercise 3
    println!(">> 3");
    let meters: i32 = 640;
    println!("{} meters is:", meters);
    let conversion_map: HashMap<&str, f32> = exercises::convert_meters_to_gb( meters as f32 );
    for (key, val) in conversion_map.iter() {
        println!("{val} {key}");
    }

    // exercise 4
    println!(">> 4");
    let output_f: f32 = exercises::convert_c_degree_to_f( 40.0 );
    println!("F: {}", output_f);
    let output_c: f32 = exercises::convert_f_to_c_degree( 104.0 );
    println!("C degree: {}", output_c);

    // exercise 5
    println!(">> 5");
    let radius: i32 = 4;
    println!("Radius: {}", radius);
    let circle_params: (f32, f32) = exercises::get_circle_params( radius as f32 );
    println!("Circle: {} {}.", circle_params.0, circle_params.1);
    
    // exercise 6
    println!(">> 6");
    let angle: f32 = 165.0;
    println!("Angle: {}",angle);
    let sector_params: (f32, f32) = exercises::get_sector_params( radius as f32, angle );
    println!("Sector: {} {}.", sector_params.0, sector_params.1);

    // exercise 7
    println!(">> 7");
    let position_y: f32 = exercises::get_ball_position_y(12.5, 0.12, 3.0, 2.963);
    println!("Ball_position_y: {}", position_y);
    
    // exercise 8
    println!(">> 8");
    println!("Compound interest:");
    let compound_interest_0: f32 = exercises::get_compound_interest( 1000.0, 1, 2, 0.05 );
    println!("1) {}", compound_interest_0);
    let compound_interest_1: f32 = exercises::get_compound_interest( 1000.0, 4, 3, 0.023 );
    println!("2) {}", compound_interest_1);

    // exercise 9
    println!(">> 9");
    println!("{:e}", 12345);
    println!("{:e}", 0.12345);
    println!("{:e}", 0.00000123);
    println!("{:.2e}", 1020304050);
    println!("Planet: radius [km], circumference [km], area [km^2], distance from the Sun [AU], mass [M]");
    println!("Jupiter: {:e}, {:e}, {:e}, {:e}, {:e}", 69_911, 439_264, 5.2, 778_000_000, 317.8);
    println!("Saturn: {:e}, {:e}, {:e}, {:e}, {:e}", 58_232, 365_882.4, 9.5, 1_400_000_000, 95.16);

    // exercise 10
    println!(">> 10");
    println!("The Earth travels distance:");
    let distance_hour = exercises::calculate_distance( 30.0, (60*60) as f32 );
    println!("{:e} km after hour", distance_hour);
    let distance_day = exercises::calculate_distance( 30.0, (24*60*60) as f32 );
    println!("{:e} km after day", distance_day);
    let distance_year = exercises::calculate_distance( 30.0, (365*24*60*60) as f32 );
    println!("{:e} km after year", distance_year);
    
}
