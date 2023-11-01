use std::collections::LinkedList;
mod utils;

fn main()
{
    //exercise 1
    println!(">> 1");
    let mut linked_list: LinkedList<i32> = LinkedList::from([7, 1, 291, 15, -2, 3, 55, 102]);
    for i in linked_list.iter() {
        print!("-> {} ", i);
    }
    let results_minimum: (i32, u32) = utils::min_search_list(linked_list);
    println!("\n min: {}, comparisons: {}", results_minimum.0, results_minimum.1);

    //exercise 3
    println!(">> 3");
    linked_list = LinkedList::from([7, 1, 291, 15, -2, 3, 55, 102, -2, 291, 34]);
    for i in linked_list.iter() {
        print!("-> {} ", i);
    }
    let results_minimum: (i32, u32, u32) = utils::min_idx_search_list(linked_list);
    println!("\n min: {}, index: {}, comparisons: {}", results_minimum.0, results_minimum.1, results_minimum.2);

    //exercise 4
    println!(">> 4");
    linked_list = LinkedList::from([7, 1, 291, 15, -2, 3, 55, 102, -2, 291, 34]);
    for i in linked_list.iter() {
        print!("-> {} ", i);
    }
    let results_minimum: (i32, u32, u32) = utils::min_idx_max_search_list(linked_list);
    println!("\n min: {}, index: {}, comparisons: {}", results_minimum.0, results_minimum.1, results_minimum.2);

    //exercise 5
    println!(">> 5");
    linked_list = LinkedList::from([7, 1, 291, 15, -2, 3, 55, 102, -2, 291, 34]);
    for i in linked_list.iter() {
        print!("-> {} ", i);
    }
    let results_maximum: (i32, u32, u32) = utils::max_idx_max_search_list(linked_list);
    println!("\n max: {}, index: {}, comparisons: {}", results_maximum.0, results_maximum.1, results_maximum.2);

    //exercise 6
    println!(">> 6");

}
