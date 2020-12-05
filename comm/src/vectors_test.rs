#[cfg(test)]
mod vectors {
    use crate::bytes::create_empty_bytes;
    use crate::vectors::{find_eq_vec_bytes, find_last_eq_bytes, modify, sub};

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

    #[test]
    fn find_last_eq_bytes_test() {
        let mut a: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let mut b = create_empty_bytes(8);
        let mut c = vec![0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x10];
        let mut d = create_empty_bytes(8);
        let mut e = vec![0x03, 0x04, 0x05, 0x06, 0x01, 0x02, 0x08, 0x10];
        let mut f = create_empty_bytes(8);
        a.append(&mut b);
        a.append(&mut c);
        a.append(&mut d);
        a.append(&mut e);
        a.append(&mut f);
        println!("a = {:#?}", a);
        let g = find_last_eq_bytes(a, 8);
        println!("g = {:#?}", g);
    }

    #[test]
    fn find_eq_vec_bytes_test() {
        let mut a: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07];
        let mut b = create_empty_bytes(8);
        let mut c = vec![0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x10];
        let mut d = create_empty_bytes(8);
        let mut e = vec![0x03, 0x04, 0x05, 0x06, 0x01, 0x02, 0x08, 0x10];
        let mut f = create_empty_bytes(8);
        a.append(&mut b);
        a.append(&mut c);
        a.append(&mut d);
        a.append(&mut e);
        a.append(&mut f);
        println!("a = {:#?}", a);
        let g = find_eq_vec_bytes(a, 8);
        println!("g = {:#?}", g);
    }
}
