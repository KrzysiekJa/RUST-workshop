use std::collections::{LinkedList, linked_list};

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

pub fn min_idx_search_list(list: LinkedList<i32>) -> (i32, u32, u32)
{
    let limit: u32 = list.len() as u32 - 1;
    let mut amount: u32 = 0;
    let mut counted: u32 = 0;
    let mut iter: linked_list::Iter<'_, i32> = list.iter();
    let mut min: Option<&i32> = iter.next();
    let mut min_index: u32 = 0;

    while amount < limit {
        let tmp = iter.next();
        if min > tmp {
            min = tmp;
            min_index = amount + 1;
        }
        counted += 1;
        amount += 1;
    }
    return (* min.unwrap(), min_index, counted);
}

pub fn min_idx_max_search_list(list: LinkedList<i32>) -> (i32, u32, u32)
{
    let limit: u32 = list.len() as u32 - 1;
    let mut amount: u32 = 0;
    let mut counted: u32 = 0;
    let mut iter: linked_list::Iter<'_, i32> = list.iter();
    let mut min: Option<&i32> = iter.next();
    let mut min_index: u32 = 0;

    while amount < limit {
        let tmp = iter.next();
        if min >= tmp {
            min = tmp;
            min_index = amount + 1;
        }
        counted += 1;
        amount += 1;
    }
    return (* min.unwrap(), min_index, counted);
}

