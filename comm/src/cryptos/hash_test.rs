#[cfg(test)]
mod md5 {
    use crate::cryptos::hash::{
        hashcode32, hashcode32_enhance, hashcode64, hashcode64_enhance, hashcode_enhance, md5,
        md516,
    };

    #[test]
    fn md5_test() {
        let str = "test".to_string();
        let md5_1 = md5(str.clone());
        let md5_2 = md5(str.clone());
        println!("test md5 1 = {}", md5_1);
        println!("test md5 2 = {}", md5_2);
        let md516_1 = md516(str.clone());
        let md516_2 = md516(str.clone());
        println!("test md516 1 = {}", md516_1);
        println!("test md516 2 = {}", md516_2);
    }

    #[test]
    fn hashcode32_test() {
        let bytes1 = "test1".as_bytes();
        let bytes2 = "test2".as_bytes();

        println!("res = {}", hashcode32(bytes1));
        println!("res = {}", hashcode32(bytes1) as u64);
        println!("res = {}", hashcode32(bytes2));
        println!("res = {}", hashcode32(bytes2) as u64);
    }

    #[test]
    fn hashcode64_test() {
        let bytes1 = "test1".as_bytes();
        let bytes2 = "test2".as_bytes();

        println!("res = {}", hashcode64(bytes1));
        println!("res = {}", hashcode64(bytes2));
    }

    #[test]
    fn hashcode_enhance_test() {
        let x1 = String::from("1");
        let x2 = String::from("100");
        let x3 = String::from("10000");
        println!("x1 = {}", hashcode32_enhance(x1.clone()) + 1);
        println!("x2 = {}", hashcode32_enhance(x2.clone()) + 1);
        println!("x3 = {}", hashcode32_enhance(x3.clone()) + 1);
        println!("x1 = {}", hashcode64_enhance(x1.clone()) + 2);
        println!("x2 = {}", hashcode64_enhance(x2.clone()) + 2);
        println!("x3 = {}", hashcode64_enhance(x3.clone()) + 2);

        println!("x3 = {:#?}", hashcode_enhance(true, x3.clone()));
        println!("x3 = {:#?}", hashcode_enhance(false, x3.clone()));
    }

    #[test]
    fn hashcode64_enhance_test() {
        let x1 = String::from("-1");
        let x2 = String::from("-2094967294");
        let x3 = String::from("-8446744073709551615");
        let x4 = String::from("18446744073709551615");
        let t1 = String::from("test1");
        let t2 = String::from("test2");
        let bytes1 = "test1".as_bytes();
        let bytes2 = "test2".as_bytes();

        println!("res1 = {}", hashcode64(bytes1));
        println!("res2 = {}", hashcode64_enhance(t1));
        println!("res3 = {}", hashcode64(bytes2));
        println!("res4 = {}", hashcode64_enhance(t2));
        println!("res5 = {}", hashcode64_enhance(x1));
        println!("res6 = {}", hashcode64_enhance(x2));
        println!("res7 = {}", hashcode64_enhance(x3));
        println!("res8 = {}", hashcode64_enhance(x4));

        let m: u64 = 1 << 63;
        println!("2^64 = {}", m);

        let uu: u32 = 1988888;
        let uu64 = uu as u64;
        let uu32 = uu64 as u32;
        println!("u1 = {}", uu);
        println!("u2 = {}", uu64);
        println!("u3 = {}", uu32);
    }
}
