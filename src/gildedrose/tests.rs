use super::*;

#[test]
pub fn foo() {
    let items = vec![Item::new("foo", 0, 0)];
    let mut rose = GildedRose::new(items);
    rose.update_quality();

    assert_eq!("fixme", rose.items[0].name);
}
