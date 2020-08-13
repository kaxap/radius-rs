mod parser_match;
mod radius;
mod radius_test;
mod vendors;

#[macro_use]
extern crate nom;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
