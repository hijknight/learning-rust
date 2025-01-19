pub fn add(left: usize, right: usize) -> usize {
    left + right
}


pub fn adds_two(a: i32) -> i32 {
    a + 2
}
// private test

fn internal_adder(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;
    // private function test
    #[test]
    fn it_works() {
        let result = internal_adder(2, 2);
        assert_eq!(result, 4);
    }


}