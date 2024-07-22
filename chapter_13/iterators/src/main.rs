fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter(); // doesn't need to be mutable, the loop below takes ownership of the iterator and makes it mutable BTS
    // for ownership, use into_iter. to iterate over mutable references, use iter_mut.

    for val in v1_iter {
        println!("got: {val}");
    }

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // we need to consume this iterator with collect

    assert_eq!(v2, vec![2, 3, 4]);
}