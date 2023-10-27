use std::collections::LinkedList;

pub fn min_search_list(list: LinkedList<i32>) -> (i32, u32)
{
    let mut amount: usize = list.len() - 1;
    let mut counted: u32 = 0;
    let mut iter: std::collections::linked_list::Iter<'_, i32> = list.iter();
    let mut min: Option<&i32> = iter.next();

    while amount > 0 {
        let tmp = iter.next();
        if min > tmp {
            min = tmp;
        }
        counted += 1;
        amount -= 1;
    }
    return (* min.unwrap(), counted);
}