use rand::Rng;
use std::collections::HashMap;

const MIN_LENGTH: usize = 3;
const MAX_LENGTH: usize = 10;
const MIN_DATA: usize = 1;
const MAX_DATA: usize = 100;

fn main() {
    let length = rand::thread_rng().gen_range(MIN_LENGTH..=MAX_LENGTH);
    let list = generate_list(&length);
    println!("the list i generated is here: {list:?}");

    let median = find_median(&list, length);
    println!("the median of this list is: {median}");

    let mode = find_mode(&list);
    println!("the mode(s) of this list are: {mode:?}");
}

fn generate_list(length: &usize) -> Vec<usize> {
    let mut list = Vec::new();
    for _i in 0..*length {
        let point = rand::thread_rng().gen_range(MIN_DATA..=MAX_DATA);
        list.push(point);
    }
    list
}

fn find_median(list: &Vec<usize>, length: usize) -> f32 {
    let pivot_index = get_pivot(length);
    if length % 2 == 0 {
        (quickselect(&list, (length - 1) / 2, pivot_index) + quickselect(&list, ((length - 1) / 2) + 1, pivot_index)) / 2.0
    } else {
        quickselect(&list, (length - 1) / 2, pivot_index)
    }
}

fn find_mode(list: &Vec<usize>) -> Vec<usize> {
    let mut values = HashMap::new();

    for i in list {
        let count = values.entry(i).or_insert(0);
        *count += 1;
    }
    
    let mut high_k = Vec::new();
    let mut high_v: u32 = 1;

    for (k, v) in values {
        if v > high_v {
            high_k.clear();
            high_k.push(*k);
            high_v = v;
        } else if v == high_v {
            high_k.push(*k);
        }
    }
    high_k
}

fn quickselect(list: &Vec<usize>, k: usize, pivot_index: usize) -> f32 {
    // base case
    if list.len() == 1 {
        list[0] as f32
    } else {
        // recurse
        let mut lesser_list: Vec<usize> = Vec::new();
        let mut greater_list: Vec<usize> = Vec::new();
        let pivot = list[pivot_index];
        for i in list {
            if i <= &pivot {
                lesser_list.push(*i);
            } else {
                greater_list.push(*i);
            }
        }
        println!("pivot: {pivot} k: {k} lesser list: {lesser_list:?} greater list: {greater_list:?}");
        let len_lesser = lesser_list.len();
        if &len_lesser > &k {
            quickselect(&lesser_list, k, get_pivot(len_lesser))
        } else {
            quickselect(&greater_list, k - &len_lesser, get_pivot(greater_list.len()))
        }
    }
}

fn get_pivot(length: usize) -> usize {
    rand::thread_rng().gen_range(0..length)
}