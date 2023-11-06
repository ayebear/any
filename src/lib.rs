mod any;

#[cfg(test)]
mod tests {
    use super::any::*;

    #[test]
    fn it_works() {
        let result = Any::from(0.1) + Any::from(2);
        assert_eq!(result, Any::from(2.1));
    }
}
