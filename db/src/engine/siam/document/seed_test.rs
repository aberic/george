#[cfg(test)]
mod seed_test {
    use crate::engine::siam::document::seed::Seed;
    use crate::engine::traits::TSeed;

    #[test]
    fn create_and_modify() {
        println!("Hello, world!");
        let l = Seed::create(vec![0x01]);
        println!("seed is {:#?}", l);
        println!("seed md516_key = {}", l.key());
        println!();
        println!("seZed is {:#?}", l);
        println!("seed md516_key = {}", l.key());
    }
}
