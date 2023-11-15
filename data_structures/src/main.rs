use std::collections::LinkedList;
use rand::Rng;
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
    println!("\n\n min: {}, comparisons: {}", results_minimum.0, results_minimum.1);

    //exercise 3
    println!(">> 3");
    linked_list = LinkedList::from([7, 1, 291, 15, -2, 3, 55, 102, -2, 291, 34]);
    for i in linked_list.iter() {
        print!("-> {} ", i);
    }
    let results_minimum: (i32, u32, u32) = utils::min_idx_search_list(linked_list.clone());
    println!("\n\n min: {}, index: {}, comparisons: {}", results_minimum.0, results_minimum.1, results_minimum.2);

    //exercise 4
    println!(">> 4");
    let results_minimum: (i32, u32, u32) = utils::min_idx_max_search_list(linked_list.clone());
    println!("min: {}, index: {}, comparisons: {}", results_minimum.0, results_minimum.1, results_minimum.2);

    //exercise 5
    println!(">> 5");
    let results_maximum: (i32, u32, u32) = utils::max_idx_max_search_list(linked_list.clone());
    println!("max: {}, index: {}, comparisons: {}", results_maximum.0, results_maximum.1, results_maximum.2);

    //exercise 6
    println!(">> 6");
    let results_maximum: (i32, i32, u32) = utils::min_max_search_list(linked_list.clone());
    println!("min: {}, max: {}, comparisons: {}", results_maximum.0, results_maximum.1, results_maximum.2);

    //exercise 7
    println!(">> 7");
    println!("Different sequences:");

    for i in linked_list.iter() {
        print!("-> {} ", i);
    }
    let sequence_sum: i32 = utils::sum_of_elements(linked_list.clone());
    println!("\n\n sum: {}", sequence_sum);

    let sequence_product: i64 = utils::product_of_elements(linked_list.clone());
    println!(" product: {}", sequence_product);

    let sequence_mean: i32 = utils::mean_of_elements(linked_list.clone());
    println!(" mean: {}", sequence_mean);

    let sequence_mean_pos: f32 = utils::mean_of_positive_elements(linked_list.clone());
    println!(" mean: {}", sequence_mean_pos);

    let sequence_multiplication: i128 = utils::multiplication_of_elements(linked_list.clone());
    println!(" multiplication: {}", sequence_multiplication);

    //exercise 8
    println!(">> 8");
    let mut vec: Vec<_> = linked_list.clone().into_iter().collect();
    vec.sort();
    let linked_list: LinkedList<_> = vec.into_iter().collect();

    for i in linked_list.iter() {
        print!("-> {} ", i);
    }

    let key: i32 = 55;
    let index: Option<u32> = utils::find_index(linked_list, key);
    println!("\nkey: {}-> idx: {}", key, index.unwrap());

    //exercise 9
    println!(">> 9");
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    let vec_1: Vec<u32> = (0..8).map(|_| rng.gen_range(0..11)).collect();
    let vec_2: Vec<u32> = (0..8).map(|_| rng.gen_range(0..11)).collect();
    println!("Vector 1: {:?}", vec_1);
    println!("Vector 2: {:?}", vec_2);

    let x_to_find: u32 = 17;
    let searching_result: bool = utils::found_if_sum_equal_x(&vec_1, &vec_2, x_to_find);
    println!("\nAre elements of vecs making sum of {} : {}", x_to_find, searching_result);

}
