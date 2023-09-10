mod exercises;

fn main() {

    // exercise 0
    exercises::exercise_types();
    
    // exercise 1
    println!(">> 1");
    exercises::throwed_ball();

    // exercise 2
    println!(">> 2");
    exercises::throwed_ball_planets();

    // exercise 3
    println!(">> 3");
    exercises::convert_meters_to_gb( 640 as f32 );

    // exercise 4
    println!(">> 4");
    let output_f: f32 = exercises::convert_c_degree_to_f( 40.0 );
    println!("{}", output_f);
    let output_c: f32 = exercises::convert_f_to_c_degree( 104.0 );
    println!("{}", output_c);

    // exercise 5
    println!(">> 5");
    let circle_params: (f32, f32) = exercises::circle_params( 4 as f32 );
    println!("{} {}", circle_params.0, circle_params.1);
    
    // exercise 6
    println!(">> 6");
    let sector_params: (f32, f32) = exercises::sector_params( 4 as f32, 165.0 );
    println!("{} {}", sector_params.0, sector_params.1);

    // exercise 7
    println!(">> 7");
    let position_y: f32 = exercises::get_ball_position_y(12.5, 0.12, 3.0, 2.963);
    println!("{}", position_y);
    
    // exercise 8
    println!(">> 8");
    let compound_interest_0: f32 = exercises::compound_interest( 1000.0, 1, 2, 0.05 );
    println!("{}", compound_interest_0);
    let compound_interest_1: f32 = exercises::compound_interest( 1000.0, 4, 3, 0.023 );
    println!("{}", compound_interest_1);
    
}
