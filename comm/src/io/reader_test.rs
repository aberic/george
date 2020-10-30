#[cfg(test)]
mod reader {
    use crate::io::reader::{read_all, read_sub_bytes};

    #[test]
    fn reader_test() {
        let s = read_all("src/examples/conf.yaml");
        println!("s = {:#?}", s);
    }

    #[test]
    fn read_sub_bytes_test() {
        println!(
            "res1 = {:#?}",
            read_sub_bytes(
                "src/examples/29f459a44fee58c7.sr"
                    .to_string(),
                448,
                8,
            )
        );
        println!(
            "res2 = {:#?}",
            read_sub_bytes(
                "src/examples/29f459a44fee58c7.sr"
                    .to_string(),
                0,
                2048,
            )
        );
    }
}
