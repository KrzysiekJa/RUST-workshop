use std::collections::LinkedList;

pub fn min_search_list(list: LinkedList<i32>) -> i32
{
    let mut amount = list.len() - 1;
    let mut iter = list.iter();
    let mut min = iter.next();

    while amount > 0 {
        let tmp = iter.next();
        if min > tmp {
            min = tmp;
        }
        amount -= 1;
    }
    return * min.unwrap();
}