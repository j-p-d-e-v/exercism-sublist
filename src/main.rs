use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn check_list<T: PartialEq>(_list_one: &[T], _list_two: &[T], _return_true: Comparison ) -> Comparison {
        let mut _return: Comparison = Comparison::Unequal;
        let mut matched_counter: usize = 0;
        let mut last_known_index: usize = 0;
        for l_item in _list_one {
            let mut has_matched: bool = false;
            for i in last_known_index.._list_two.len() {
                let s_item: &T = &_list_two[i];
                if l_item == s_item {
                    has_matched = true;
                    last_known_index += 1;
                    break;
                }                
                else {
                    last_known_index = 0;
                }
            }
            if has_matched {
                matched_counter += 1;
            }
            else {
                matched_counter = 0;
            }
            if matched_counter == _list_two.len() {
                _return = _return_true;
                break;
            }
        }
        _return
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.is_empty() && _second_list.is_empty() {
        Comparison::Equal
    }
    else if !_first_list.is_empty() && _second_list.is_empty() {
        Comparison::Superlist
    }
    else if _first_list.is_empty() && !_second_list.is_empty() {
        Comparison::Sublist
    }
    else if _first_list.len() == _second_list.len() {
        check_list(_first_list,_second_list,Comparison::Equal)
    }
    else if _first_list.len() < _second_list.len() {
        check_list(_second_list,_first_list,Comparison::Sublist)
    }
    else {
        check_list(_first_list,_second_list,Comparison::Superlist)
    }
}



fn empty_lists() {
    let list_one: &[i32] = &[];
    let list_two: &[i32] = &[];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Equal;
    assert_eq!(output, expected);
}
fn empty_list_within_non_empty_list() {
    let list_one: &[i32] = &[];
    let list_two: &[i32] = &[1, 2, 3];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Sublist;
    assert_eq!(output, expected);
}
fn non_empty_list_contains_empty_list() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Superlist;
    assert_eq!(output, expected);
}

fn list_equals_itself() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[1, 2, 3];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Equal;
    assert_eq!(output, expected);
}

fn different_lists() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[2, 3, 4];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}

fn false_start() {
    let list_one: &[i32] = &[1, 2, 5];
    let list_two: &[i32] = &[0, 1, 2, 3, 1, 2, 5, 6];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Sublist;
    assert_eq!(output, expected);
}

fn consecutive() {
    let list_one: &[i32] = &[1, 1, 2];
    let list_two: &[i32] = &[0, 1, 1, 1, 2, 1, 2];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Sublist;
    assert_eq!(output, expected);
}

fn sublist_at_start() {
    let list_one: &[i32] = &[0, 1, 2];
    let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Sublist;
    assert_eq!(output, expected);
}

fn sublist_in_middle() {
    let list_one: &[i32] = &[2, 3, 4];
    let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Sublist;
    assert_eq!(output, expected);
}

fn sublist_at_end() {
    let list_one: &[i32] = &[3, 4, 5];
    let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Sublist;
    assert_eq!(output, expected);
}

fn at_start_of_superlist() {
    let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
    let list_two: &[i32] = &[0, 1, 2];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Superlist;
    assert_eq!(output, expected);
}

fn in_middle_of_superlist() {
    let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
    let list_two: &[i32] = &[2, 3];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Superlist;
    assert_eq!(output, expected);
}

fn at_end_of_superlist() {
    let list_one: &[i32] = &[0, 1, 2, 3, 4, 5];
    let list_two: &[i32] = &[3, 4, 5];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Superlist;
    assert_eq!(output, expected);
}


fn first_list_missing_element_from_second_list() {
    let list_one: &[i32] = &[1, 3];
    let list_two: &[i32] = &[1, 2, 3];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}

fn second_list_missing_element_from_first_list() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[1, 3];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}

fn first_list_missing_additional_digits_from_second_list() {
    let list_one: &[i32] = &[1, 2];
    let list_two: &[i32] = &[1, 22];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}

fn order_matters_to_a_list() {
    let list_one: &[i32] = &[1, 2, 3];
    let list_two: &[i32] = &[3, 2, 1];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}

fn same_digits_but_different_numbers() {
    let list_one: &[i32] = &[1, 0, 1];
    let list_two: &[i32] = &[10, 1];
    let output = sublist(list_one, list_two);
    let expected = Comparison::Unequal;
    assert_eq!(output, expected);
}

fn main(){
    
    println!("first_list_missing_element_from_second_list");
    first_list_missing_element_from_second_list();
    println!("false_start");
    false_start();
    println!("empty_lists");
    empty_lists();
    println!("empty_list_within_non_empty_list");
    empty_list_within_non_empty_list();
    println!("non_empty_list_contains_empty_list");
    non_empty_list_contains_empty_list();
    println!("list_equals_itself");
    list_equals_itself();
    println!("different_lists");
    different_lists();
    println!("consecutive");
    consecutive();
    println!("sublist_at_start");
    sublist_at_start();
    println!("sublist_in_middle");
    sublist_in_middle();
    println!("sublist_at_end");
    sublist_at_end();
    println!("at_start_of_superlist");
    at_start_of_superlist();
    println!("in_middle_of_superlist");
    in_middle_of_superlist();
    println!("at_end_of_superlist");
    at_end_of_superlist();
    println!("second_list_missing_element_from_first_list");
    second_list_missing_element_from_first_list();
    println!("first_list_missing_additional_digits_from_second_list");
    first_list_missing_additional_digits_from_second_list();
    println!("order_matters_to_a_list");
    order_matters_to_a_list();
    println!("same_digits_but_different_numbers");
    same_digits_but_different_numbers();
    println!("Passed to all test cases.");
}