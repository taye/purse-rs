#[macro_use]
pub mod list;
pub mod iterator;
mod node;

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn out_of_bounds_index() {
    let empty = list::List::<()>::empty();

    empty[0]
}


#[test]
fn concat_mut() {
    use std::sync::Arc;

    let mut list1 = purse_list!['a', 'b'];
    let list2 = purse_list!['c', 'd'];

    assert_eq!(Arc::strong_count(&list1.head.as_ref().unwrap()), 1);

    list1.concat_mut(&list2);

    assert_eq!(list1, purse_list!['a', 'b', 'c', 'd']);

    let mut list3 = purse_list!['!'];

    // in a singleton list, tail is a clone of head
    assert_eq!(Arc::strong_count(&list2.head.as_ref().unwrap()), 2);

    list3.concat_mut(&list2);
    assert_eq!(list3, purse_list!['!', 'c', 'd']);
}
