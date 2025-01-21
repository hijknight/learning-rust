#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust,
safe, fast, productive.
pick three";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "hello";
        let contents = "\
hello there john,
HelLo there Johnny";

        assert_eq!(vec!["hello there john,", "HelLo there Johnny"], search_insensitive(query, contents));
    }

}