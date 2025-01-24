use std::thread;
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}



fn main() {
    let mut rec_lst= [
        Rectangle { width: 10, height: 3 },
        Rectangle { width: 13, height: 14 },
        Rectangle { width: 3, height: 5 },
    ];

    rec_lst.sort_by_key(|r| r.width);
    println!("{rec_lst:?}")

    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {list:?}");
    //
    // thread::spawn(move || println!("From thread: {list:?}"))
    //     .join().unwrap();

    // let mut list = vec![1, 2, 3];
    //
    // println!("Before defining closure: {list:?}");
    //
    // let mut borrows_mutably = || list.push(7);
    // // no println! because of mutable borrow
    // borrows_mutably();
    // println!("After calling closure: {list:?}");


    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {list:?}");
    //
    // let only_borrows = || println!("From closure: {list:?}");
    //
    // println!("Before calling closure: {list:?}");
    // only_borrows();
    // println!("After calling closure: {list:?}")
}
