use std::collections::{LinkedList};
mod utils;

fn main()
{
    //exercise 1
    println!(">> 1");
    let linked_list: LinkedList<i32> = LinkedList::from([7, 1, 291, 15, -2, 3, 55, 102]);
    for i in linked_list.iter() {
        print!("-> {} ", i);
    }
    let results_minimum = utils::min_search_list(linked_list);
    println!("\n min: {}, O(n): {}", results_minimum.0, results_minimum.1);

    //exercise 2
    println!(">> 2");

}
