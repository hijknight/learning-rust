use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data {}", self.data);
//     }
// }

fn main() {
    let c = CustomSmartPointer {
        data: String::from("Rust"),
    };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped")
}
