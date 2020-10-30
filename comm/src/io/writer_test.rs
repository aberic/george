#[cfg(test)]
mod reader {
    use crate::io::file::create_file_str;
    use crate::io::writer::{write_append_bytes, write_seek_u8s};

    #[test]
    fn reader_test() {
        create_file_str("src/test/file/g.txt", true).unwrap();
        match write_append_bytes(
            "src/test/file/g.txt".to_string(),
            vec![
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a,
            ],
        ) {
            Ok(()) => {
                let vs: Vec<u8> = vec![0x0b, 0x0c, 0x0d, 0x0e];
                match write_seek_u8s("src/test/file/g.txt".to_string(), 3, vs.as_slice()) {
                    Err(err) => println!("err = {}", err),
                    _ => {}
                }
            }
            Err(err) => println!("err = {}", err),
        }
    }
}
