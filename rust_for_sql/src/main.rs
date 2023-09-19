mod simple_sql;
mod time_stamp;

fn main()
{
    simple_sql::run_sql_task();
    println!("");
    time_stamp::get_local_time();
    println!("");
    time_stamp::get_time_stamp(-8, 2)
}
