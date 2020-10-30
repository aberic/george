#[cfg(test)]
mod sm3 {
    use crate::cryptos::sm3;

    #[test]
    fn sm3_test() {
        let str = "test".to_string();
        let sm3_1 = sm3::hash(str.clone());
        let sm3_2 = sm3::hash(str.clone());
        println!("test sm3 1 = {}", sm3_1);
        println!("test sm3 2 = {}", sm3_2);
        match hex::decode(sm3_1) {
            Ok(v8) => println!("u8s = {:#?}", v8.as_slice()),
            Err(err) => println!("err = {}", err.to_string()),
        }
    }
}
