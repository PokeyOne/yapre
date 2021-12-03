// Intentionally not including png related stuff here because there is already
// a library that handles it and it is best to not include that dependency
// in every little thing that needs any io.

mod obj;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
