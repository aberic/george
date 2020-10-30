#[cfg(test)]
mod vectors {
    use crate::vectors::{modify, sub};

    #[test]
    fn modify_test() {
        let x: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x10];
        let start = 3;
        let y: Vec<u8> = vec![0x20, 0x21, 0x22, 0x23, 0x24];
        let z = modify(x.clone(), y, start);
        println!("x = {:#?}\ny = {:#?}", x, z)
    }

    #[test]
    fn sub_test() {
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        println!("sub = {:#?}", sub(vec, 2, 5));

        let x: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x10];
        println!("sub = {:#?}", sub(x.clone(), 2, 5));
        println!("x = {:#?}", x);
    }
}
