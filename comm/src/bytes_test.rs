#[cfg(test)]
mod bytes {
    use crate::bytes;

    #[test]
    fn create_empty_bytes_test() {
        let x = bytes::create_empty_bytes(20);
        println!("x = {:#?}", x)
    }

    #[test]
    fn bytes_split_test() {
        let mut vec = vec![1, 2, 3];
        let vec2 = vec.split_off(1);
        println!("vec = {:#?}\nvec2 = {:#?}", vec, vec2);
        assert_eq!(vec, [1]);
        assert_eq!(vec2, [2, 3]);
    }
}
