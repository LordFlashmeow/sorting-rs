extern crate sorting;

use sorting::*;

#[test]
fn does_not_remove_in_order_list() {
    let mut sorted_vector = vec![1,2,3,4,5];

    sorted_vector.stalinsort();

    assert_eq!(sorted_vector, vec![1,2,3,4,5])
}

#[test]
fn removes_all_items_except_first_if_reverse_ordered() {
    let mut vector = vec![5,4,3,2,1];

    vector.stalinsort();

    assert_eq!(vector, vec![5])
}

#[test]
fn removes_out_of_order_items() {
    let mut vector = vec![1,2,5,4,6];
    vector.stalinsort();

    assert_eq!(vector, vec![1,2,5,6])
}