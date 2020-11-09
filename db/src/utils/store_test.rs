use crate::utils::comm::{Category, IndexType, LevelType};
use crate::utils::store::{head, FileHeader, Tag};

#[test]
fn head_test() {
    let head1 = head(FileHeader::create(
        Tag::Bootstrap,
        Category::Memory,
        LevelType::Small,
        IndexType::Siam,
        0x00,
    ));
    let head2 = head(FileHeader::create(
        Tag::Database,
        Category::Document,
        LevelType::Large,
        IndexType::Siam,
        0x01,
    ));
    println!("head1 = {:#?}", head1);
    println!("head2 = {:#?}", head2);

    assert_eq!(0x20, head1.get(0).unwrap().clone());
    assert_eq!(0x19, head1.get(1).unwrap().clone());
    assert_eq!(0x00, head1.get(2).unwrap().clone());
    assert_eq!(0x00, head1.get(3).unwrap().clone());
    assert_eq!(0x00, head1.get(4).unwrap().clone());
    assert_eq!(0x00, head1.get(5).unwrap().clone());
    assert_eq!(0x00, head1.get(6).unwrap().clone());
    assert_eq!(0x00, head1.get(7).unwrap().clone());
    assert_eq!(0x00, head1.get(8).unwrap().clone());
    assert_eq!(0x02, head1.get(30).unwrap().clone());
    assert_eq!(0x19, head1.get(31).unwrap().clone());

    assert_eq!(0x20, head2.get(0).unwrap().clone());
    assert_eq!(0x19, head2.get(1).unwrap().clone());
    assert_eq!(0x01, head2.get(2).unwrap().clone());
    assert_eq!(0x01, head2.get(3).unwrap().clone());
    assert_eq!(0x01, head2.get(4).unwrap().clone());
    assert_eq!(0x00, head2.get(5).unwrap().clone());
    assert_eq!(0x00, head2.get(6).unwrap().clone());
    assert_eq!(0x00, head2.get(7).unwrap().clone());
    assert_eq!(0x01, head2.get(8).unwrap().clone());
    assert_eq!(0x02, head2.get(30).unwrap().clone());
    assert_eq!(0x19, head2.get(31).unwrap().clone());

    // assert_eq!(0x19, head1.pop().unwrap());
    // assert_eq!(0x02, head1.pop().unwrap());
    // let mut index = 0;
    // while index < 21 {
    //     head1.pop().unwrap();
    //     index+=1
    // }
    // assert_eq!(0x00, head1.pop().unwrap());
    // assert_eq!(0x00, head1.pop().unwrap());
    // assert_eq!(0x00, head1.pop().unwrap());
    // assert_eq!(0x00, head1.pop().unwrap());
    // assert_eq!(0x00, head1.pop().unwrap());
    // assert_eq!(0x00, head1.pop().unwrap());
    // assert_eq!(0x00, head1.pop().unwrap());
    // assert_eq!(0x19, head1.pop().unwrap());
    // assert_eq!(0x20, head1.pop().unwrap());
}
