use chrono::{DateTime, FixedOffset, Local, Utc};

pub fn get_local_time()
{
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    println!("Local time now is: {}", local_time);
    println!("Utc time now is: {}", utc_time);
}

fn check_timezone(time_zone: i32) -> FixedOffset
{
    if time_zone < 0 {
        return  FixedOffset::east(-time_zone);
    }
    return FixedOffset::west(time_zone);
}

pub fn get_time_stamp(zone_number_0: i32, zone_number_1: i32)
{
    let local_time = Local::now();
    let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
    let timezone_0 = check_timezone(zone_number_0 * 3600);
    let timezone_1 = check_timezone(zone_number_1 * 3600);
    let timezone_diff = timezone_0.local_minus_utc() - timezone_1.local_minus_utc();

    println!("Local time now is {}", local_time);
    println!("UTC time now is {}", utc_time);
    println!("Time in zone 1 is {}", utc_time.with_timezone(&timezone_0));
    println!("Time in zone 2 now is {}", utc_time.with_timezone(&timezone_1));
    println!("Difference between timezones is {} h", timezone_diff/(60*60));
}
