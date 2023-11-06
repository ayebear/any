mod any;

#[cfg(test)]
mod tests {
    use super::any::*;

    #[test]
    fn add() {
        let result = Any::from(0.1) + Any::from(2);
        assert_eq!(result, Any::from(2.1));
        let result = Any::from("count: ") + Any::from(100);
        assert_eq!(result, Any::from("count: 100"));
    }

    #[test]
    fn sub() {
        let result = Any::from(0.1) - Any::from(2);
        assert_eq!(result, Any::from(-1.9));
    }

    #[test]
    fn mul() {
        let result = Any::from(0.1) * Any::from(2);
        assert_eq!(result, Any::from(0.2));
        let result = Any::from("abc") * Any::from(10);
        assert_eq!(result, Any::from("abcabcabcabcabcabcabcabcabcabc"));
        let s: Any = "hello world!".into();
        let n: Any = 2.into();
        assert_eq!(s * n, Any::from("hello world!hello world!"));
    }

    #[test]
    fn div() {
        let result = Any::from(5) / Any::from(2);
        assert_eq!(result, Any::from(2.5));
        let result = Any::from("hello there world!") / Any::from(" ");
        assert_eq!(result, Any::from(["hello", "there", "world!"]));
        let result = Any::from("hello there world!") / Any::from(3);
        assert_eq!(
            result,
            Any::from(["hel", "lo ", "the", "re ", "wor", "ld!"])
        );
    }
}
