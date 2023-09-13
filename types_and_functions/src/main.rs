use std::collections::HashMap;
mod exercises;

fn main() {

    // exercise 0
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
    println!("{}", output_f);
    let output_c: f32 = exercises::convert_f_to_c_degree( 104.0 );
    println!("{}", output_c);

    // exercise 5
    println!(">> 5");
    let circle_params: (f32, f32) = exercises::get_circle_params( 4 as f32 );
    println!("{} {}", circle_params.0, circle_params.1);
    
    // exercise 6
    println!(">> 6");
    let sector_params: (f32, f32) = exercises::get_sector_params( 4 as f32, 165.0 );
    println!("{} {}", sector_params.0, sector_params.1);

    // exercise 7
    println!(">> 7");
    let position_y: f32 = exercises::get_ball_position_y(12.5, 0.12, 3.0, 2.963);
    println!("{}", position_y);
    
    // exercise 8
    println!(">> 8");
    let compound_interest_0: f32 = exercises::get_compound_interest( 1000.0, 1, 2, 0.05 );
    println!("{}", compound_interest_0);
    let compound_interest_1: f32 = exercises::get_compound_interest( 1000.0, 4, 3, 0.023 );
    println!("{}", compound_interest_1);
    
}
