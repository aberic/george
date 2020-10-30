#[cfg(test)]
mod view_test {
    use std::error::Error;

    use crate::engine::traits::TDescription;
    use crate::engine::view::View;
    use crate::utils::comm::{Category, IndexType, LevelType};

    #[test]
    fn create() {
        let view: View = View::create(
            String::from("database_id"),
            String::from("name"),
            String::from("comment"),
            IndexType::Siam,
            Category::Memory,
            LevelType::Small,
        );
        // println!("view is {:#?}", view);
        println!("view id = {}", view.id());
        println!("view name = {}", view.name());
        println!("view comment = {}", view.comment());
        println!("view category = {:#?}", view.category());
        println!("view levelType = {:#?}", view.level());
        println!("view create_time = {:#?}", view.create_time());
        println!();
    }

    #[test]
    fn description() {
        let mut view: View = View::create(
            String::from("database_id"),
            String::from("name"),
            String::from("comment"),
            IndexType::Siam,
            Category::Memory,
            LevelType::Small,
        );
        let d = view.description();
        println!(
            "view = {}, {}, {}, {}",
            view.id(),
            view.name(),
            view.comment(),
            view.create_time().num_nanoseconds().unwrap().to_string()
        );
        println!("d = {:#?}", d);

        let mut view1: View = View::create(
            String::from("database_id"),
            String::from("name1"),
            String::from("comment1"),
            IndexType::Siam,
            Category::Document,
            LevelType::Large,
        );
        let d1 = view1.description();
        println!("d1 = {:#?}", d1);
        println!(
            "view1 = {}, {}, {}, {}",
            view1.id(),
            view.name(),
            view1.comment(),
            view1.create_time().num_nanoseconds().unwrap().to_string()
        );
        view1.recover(d).unwrap();
        println!(
            "view1 = {}, {}, {}, {}",
            view1.id(),
            view.name(),
            view1.comment(),
            view1.create_time().num_nanoseconds().unwrap().to_string()
        );
    }

    #[test]
    fn put() {
        let view: View = View::create(
            String::from("database_id"),
            String::from("name"),
            String::from("comment"),
            IndexType::Siam,
            Category::Memory,
            LevelType::Small,
        );
        view.create_index("database_id".to_string(), String::from("1"), false)
            .unwrap();
        // let seed = Arc::new(Seed::new_seed(String::from("md516"), vec![0, 0, 7]));
        let irp = view.put(
            String::from("md516"),
            String::from("view1 tValue").into_bytes(),
        );
        match irp {
            Err(ie) => println!("res1 is {:#?}", ie.source().unwrap().to_string()),
            _ => {}
        }
        let irp = view.put(
            String::from("md516"),
            String::from("view2 tValue").into_bytes(),
        );
        match irp {
            Err(ie) => println!("res2 is {:#?}", ie.source().unwrap().to_string()),
            _ => {}
        }
        let irg = view.get(String::from("md516"));
        match irg {
            Ok(vu8) => {
                // println!("u is {:#?}", vu8);
                println!("u is {:#?}", String::from_utf8(vu8).unwrap().as_str())
            }
            Err(ie) => println!("res4 is {:#?}", ie.source().unwrap().to_string()),
        }
        let irp = view.set(
            String::from("md516"),
            String::from("view3 tValue").into_bytes(),
        );
        match irp {
            Err(ie) => println!("res3 is {:#?}", ie.source().unwrap().to_string()),
            _ => {}
        }
        let irg = view.get(String::from("md516"));
        match irg {
            Ok(vu8) => {
                // println!("u is {:#?}", vu8);
                println!("u is {:#?}", String::from_utf8(vu8).unwrap().as_str())
            }
            Err(ie) => println!("res4 is {:#?}", ie.source().unwrap().to_string()),
        }
    }
}
