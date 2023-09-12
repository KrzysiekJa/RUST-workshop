use std::f32::consts::PI;


pub fn exercise_types()
{
    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);
}

pub fn get_throwed_ball_pos(v_0: f32, g: f32, t_array: [f32; 3]) -> [f32; 3]
{
    let mut h_array: [f32; 3] = [0.0; 3];

    for i in 0..t_array.len() {
        let tmp_h: f32 = v_0 * t_array[i] - 0.5 * g * f32::powf(t_array[i], 2.0);
        h_array[i] = tmp_h;
    }
    return h_array;
}

pub fn get_throwed_ball_pos_planets()
{
    let v_0: f32 = 3.44;
    let gravities = [3.7, 8.8, 9.8, 3.7, 0.27, 24.7, 10.5, 9.0, 11.7];
    let t_list = [0.54, 0.1, 0.235];

    for g in gravities{
        for t in t_list {
            let h: f32 = v_0 * t - 0.5 * g * f32::powf(t, 2.0);
            println!("{}", h);
        }
    }
}

pub fn convert_meters_to_gb(meters: f32)
{
    let in_coeff:f32 = 39.37;
    let ft_coeff:f32 = 3.2808;
    let yr_coeff:f32 = 1.0936;
    let mi_coeff:f32 = 0.0006213712;

    println!("{} inches", meters * in_coeff);
    println!("{} feets", meters * ft_coeff);
    println!("{} yards", meters * yr_coeff);
    println!("{} milles", meters * mi_coeff);
}

pub fn convert_c_degree_to_f(c_degree: f32) -> f32
{
    return 9.0/5.0 * c_degree + 32.0;
}

pub fn convert_f_to_c_degree(f: f32) -> f32
{
    return 5.0/9.0 * (f - 32.0);
}

pub fn get_circle_params(radius: f32) -> (f32, f32)
{
    return ( PI * f32::powf( radius, 2.0 ), 2. * PI * radius );
}

pub fn get_sector_params(radius: f32, angle: f32) -> (f32, f32)
{
    let proportion = angle / 360.;
    return ( PI * f32::powf( radius, 2.0 ) * proportion, 2. * PI * radius * proportion );
}

pub fn get_ball_position_y(h_0: f32, afla: f32, v_0: f32, x: f32) -> f32
{
    let g: f32 = 9.81;
    let position_change: f32 = x * afla.tan() - 
            (
                ( g * f32::powf(x, 2.0) ) / 
                ( 2.0 * f32::powf(v_0, 2.0) * f32::powf(afla.cos(), 2.0) )
            );
    return h_0 + position_change;
}

pub fn get_compound_interest(k_0: f32, m: i32, n: i32, r: f32) -> f32
{
    let mut change_param: f32 = 1.0 + (r / (m as f32));
    change_param = f32::powf( change_param, (m*n) as f32 );
    return k_0 * change_param;
}
