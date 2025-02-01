
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method impls goes here
}

fn main() {}



//
// static mut COUNTER: u32 = 0;

// fn add_to_count(inc: u32) {
//     unsafe {
//         COUNTER += inc;
//     }
// }
//
// fn main() {
//     add_to_count(3);
//
//     unsafe {
//         println!("COUNTER = {COUNTER}")
//     }
// }


// extern "C" {
//     fn abs(input: i32) -> i32;
// }
//
// fn main() {
//
//     unsafe {
//         println!("absolute value of -3 according to C: {}", abs(-3));
//     }



    // let address = 0x012345usize;
    // let r = address as *const i32;

    // let mut num = 5;
    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;
    //
    // unsafe {
    //     println!("r1 is: {}", *r1);
    //     println!("r2 is: {}", *r2);
    // }

