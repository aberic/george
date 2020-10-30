#[cfg(test)]
mod seed_test {
    use crate::engine::siam::memory::seed::Seed;
    use crate::engine::traits::TSeed;

    #[test]
    fn create_and_modify() {
        println!("Hello, world!");
        let mut l = Seed::create(String::from("tKey"));
        // println!("seed is {:?}", l);
        // println!("seed is {:#?}", l);
        println!("seed is {:#?}", l);
        println!("seed key = {}", l.key());
        println!("seed value = {:#?}", l.value());
        println!();
        l.modify(String::from("tValue2").into_bytes());
        println!("seed is {:#?}", l);
        println!("seed key = {}", l.key());
        println!(
            "seed value = {}",
            String::from_utf8(l.value().unwrap().clone())
                .unwrap()
                .as_str()
        );
    }
}
