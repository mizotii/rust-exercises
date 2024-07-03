fn main() {

}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // turns string into array of bytes

    for (i, &item) in bytes.iter().enumerate() { // iterates over said array of bytes, where i is the index and item is the byte
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
