#[cfg(test)]
mod errors_index_test {
    use std::error::Error;

    use crate::errors::children::{DataExistError, DataNoExistError};
    use crate::errors::entrances::{GeorgeResult, GeorgeError};

    fn index_ok() -> GeorgeResult<u32> {
        Ok(500)
    }

    fn index_exist_err() -> GeorgeResult<u32> {
        Err(GeorgeError::DataExistError(DataExistError))
    }

    fn index_no_exist_err() -> GeorgeResult<u32> {
        Err(GeorgeError::DataNoExistError(DataNoExistError))
    }

    fn index_test1() -> GeorgeResult<u32> {
        let v = index_ok()?;
        Ok(v)
    }

    fn index_test2() -> GeorgeResult<u32> {
        // let x = index_exist_err()?;
        Ok(100)
    }

    fn index_test3() -> GeorgeResult<u32> {
        // let x = index_no_exist_err()?;
        Ok(100)
    }

    fn matches(ir: GeorgeResult<u32>) {
        match ir {
            Ok(u) => println!("u is {}", u),
            Err(ie) => println!("res is {:#?}", ie.source().unwrap().to_string()),
        }
    }

    #[test]
    fn index() {
        let res = index_ok();
        matches(res)
    }

    #[test]
    fn index_exist() {
        let res = index_exist_err();
        matches(res)
    }

    #[test]
    fn index_no_exist() {
        let res = index_no_exist_err();
        matches(res)
    }

    #[test]
    fn index1() {
        let res = index_test1();
        matches(res)
    }

    #[test]
    fn index2() {
        let res = index_test2();
        matches(res)
    }

    #[test]
    fn index3() {
        let res = index_test3();
        matches(res)
    }
}
