use rand::Rng;

const MIN_LENGTH: usize = 3;
const MAX_LENGTH: usize = 10;
const MIN_DATA: usize = 1;
const MAX_DATA: usize = 100;

fn main() {
    let length = rand::thread_rng().gen_range(MIN_LENGTH..=MAX_LENGTH);
    let list = generate_list(&length);

    println!("{list:?}");

    let median = find_median(&list, &length);

    let mode = find_mode(&list);
}

fn generate_list(length: &usize) -> Vec<usize> {
    let mut list = Vec::new();
    for i in 0..*length {
        let point = rand::thread_rng().gen_range(MIN_DATA..=MAX_DATA);
        list.push(point);
    }
    list
}

fn find_median(list: &Vec<usize>, length: &usize) -> f32 {
    if length % 2 != 1 {
        quickselect(&list, length / 2, get_pivot(length))
    } else {
        (quickselect(&list, length / 2, get_pivot(length)) + quickselect(&list, (length / 2) + 1, get_pivot(length))) / 2.0
    }
}

fn find_mode(list: &Vec<usize>) -> usize {
    1
}

fn quickselect(list: &Vec<usize>, k: usize, pivot_index: usize) -> f32 {
    let pivot: &usize = &list[pivot_index];
    let mut lesser_list = Vec::new();
    let mut greater_list = Vec::new();
    for i in list {
        if i <= pivot {
            lesser_list.push(*i);
        } else {
            greater_list.push(*i);
        }
    }
    if lesser_list.len() >= k {
        quickselect(&lesser_list, k, get_pivot(lesser_list.len()))
    } else {
        0.0
    }
    0.0
}

fn get_pivot(length: &usize) -> usize {
    rand::thread_rng().gen_range(0..*length)
}