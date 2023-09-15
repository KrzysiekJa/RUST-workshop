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
