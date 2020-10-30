#[cfg(test)]
mod path {
    use crate::utils::path::{data_path, database_path, view_path, bootstrap_file_path, index_file_path};

    # [test]
    fn path_test () {
        println!("data_path = {}", data_path());
        println!("database_path = {}", database_path(String::from("database")));
        println!("view_path = {}", view_path(String::from("database"), String::from("view")));
        println!("bootstrap_file_path = {}", bootstrap_file_path());
        println!("index_file_path = {}", index_file_path(String::from("database"), String::from("view"), String::from("index")));
    }
}