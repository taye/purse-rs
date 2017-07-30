#[macro_use]
pub mod list;
pub mod iterator;

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn out_of_bounds_index() {
    let empty = list::List::<()>::empty();

    empty[0]
}
