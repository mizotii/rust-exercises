use rand::Rng;

const MIN_LENGTH: usize = 3;
const MAX_LENGTH: usize = 10;
const MIN_DATA: usize = 1;
const MAX_DATA: usize = 100;

fn main() {
    let length = rand::thread_rng().gen_range(MIN_LENGTH..=MAX_LENGTH);
    let list = generate_list(&length);
    println!("the list i generated is here: {list:?}");

    let median = find_median(list, length);
    println!("the median of this list is: {median}");
}

fn generate_list(length: &usize) -> Vec<usize> {
    let mut list = Vec::new();
    for i in 0..*length {
        let point = rand::thread_rng().gen_range(MIN_DATA..=MAX_DATA);
        list.push(point);
    }
    list
}

fn find_median(list: Vec<usize>, length: usize) -> f32 {
    let mut pivot_index = get_pivot(length);
    if length % 2 == 0 {
        (quickselect(&list, (length - 1) / 2, pivot_index) + quickselect(&list, ((length - 1) / 2) + 1, pivot_index)) / 2.0
    } else {
        quickselect(&list, (length - 1) / 2, pivot_index)
    }
}

fn quickselect(list: &Vec<usize>, mut k: usize, pivot_index: usize) -> f32 {
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
        println!("{pivot} {lesser_list:?} {greater_list:?}");
        let converted_len_lesser = lesser_list.len() - 1;
        if &converted_len_lesser >= &k {
            quickselect(&lesser_list, k, get_pivot(converted_len_lesser))
        } else {
            quickselect(&greater_list, k - &converted_len_lesser, get_pivot(greater_list.len()))
        }
    }
}

fn get_pivot(length: usize) -> usize {
    rand::thread_rng().gen_range(0..length)
}