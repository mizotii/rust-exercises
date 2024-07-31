use std::slice;

static HELLO_WORLD: &str = "hello, world!";
static mut COUNTER: u32 = 0;

unsafe fn dangerous() {}

// we can call this from safe rust -- it creates a safe abstraction
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut[i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

unsafe trait Foo {}

unsafe impl Foo for i32 {}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // creates a slice 10,000 items long at an arbitrary memory location

    let address = 0x012345usize;
    let r = address as *mut i32;

    // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    //

    unsafe {
        dangerous();
    }

    //

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    //

    unsafe {
        println!("absolute value of -3 according to C: {}", abs(-3));
    }

    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("just called a rust function from C!");
    }

    //

    println!("name is: {HELLO_WORLD}");

    //

    add_to_count(3);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}
