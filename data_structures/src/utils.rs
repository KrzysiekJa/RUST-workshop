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

pub fn max_idx_max_search_list(list: LinkedList<i32>) -> (i32, u32, u32)
{
    let limit: u32 = list.len() as u32 - 1;
    let mut amount: u32 = 0;
    let mut counted: u32 = 0;
    let mut iter: linked_list::Iter<'_, i32> = list.iter();
    let mut max: Option<&i32> = iter.next();
    let mut max_index: u32 = 0;

    while amount < limit {
        let tmp = iter.next();
        if max <= tmp {
            max = tmp;
            max_index = amount + 1;
        }
        counted += 1;
        amount += 1;
    }
    return (* max.unwrap(), max_index, counted);
}

pub fn min_max_search_list(list: LinkedList<i32>) -> (i32, i32, u32)
{
    let limit: u32 = list.len() as u32 - 1;
    let mut counted: u32 = 0;
    let mut iter: linked_list::Iter<'_, i32> = list.iter();
    let mut min: Option<&i32> = iter.next();
    let mut max: Option<&i32> = min;

    while counted < limit {
        let tmp = iter.next();
        if min > tmp {
            min = tmp;
        }
        if max < tmp {
            max = tmp;
        }
        counted += 1;
    }
    return (* min.unwrap(), * max.unwrap(), counted);
}

pub fn sum_of_elements(list: LinkedList<i32>) -> i32
{
    return list.iter().sum();
}

pub fn product_of_elements(list: LinkedList<i32>) -> i64
{
    let product: i64 = list.iter().map(|x| *x as i64).product();
    return product;
}

pub fn mean_of_elements(list: LinkedList<i32>) -> i32
{
    let sum: i32 = list.iter().sum();
    return sum / list.len() as i32;
}

pub fn mean_of_positive_elements(list: LinkedList<i32>) -> f32
{
    let sum: i32 = list.iter().map( |x: &i32|
        match *x > 0 {
            true  => *x,
            false => 0,
        }).sum();
    return sum as f32 / ( list.len() as f32 );
}
