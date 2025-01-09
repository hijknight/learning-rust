fn main() {
    let y = {
        let x: u8 = 5;
        x + 1
    };
    println!("the value of y is {}", y);

    another_function()

}
// define another function
fn another_function() {
    println!("Another function!");
}