use set_assertions::{assert_eq_sets, assert_unique};

#[test]
fn test_assert_eq_sets() {
    let a = vec![1, 2, 3];
    let b = vec![3, 2, 1];
    assert_eq_sets!(a, b);
}

#[test]
fn test_assert_eq_sets_with_refs() {
    let a = vec![1, 2, 3];
    let b = vec![3, 2, 1];
    assert_eq_sets!(&a, &b);
}

#[test]
#[should_panic(expected = r#"Sets are not equal:
 left only = {2},
right only = {3},
   in both = {1}"#)]
fn test_assert_eq_sets_fails() {
    let a = vec![1, 2];
    let b = vec![1, 3];
    assert_eq_sets!(a, b);
}

#[test]
#[should_panic(expected = r#"Sets are not equal: [1, 2] != [1, 3]
 left only = {2},
right only = {3},
   in both = {1}"#)]
fn test_assert_eq_sets_fails_with_message() {
    let a = vec![1, 2];
    let b = vec![1, 3];
    assert_eq_sets!(&a, &b, "{:?} != {:?}", a, b);
}

#[test]
fn test_assert_unique() {
    let items = vec![1, 2, 3];
    assert_unique!(items);
}

#[test]
#[should_panic(expected = r#"Items are not unique:
duplicates = [(3, 4), (2, 2)],
    unique = {1}"#)]
fn test_assert_unique_fails() {
    let items = vec![1, 2, 2, 4, 4, 4];
    assert_unique!(items);
}

#[test]
#[should_panic(expected = r#"Items are not unique: Dummy message: xyzzy
duplicates = [(3, 4), (2, 2)],
    unique = {1}"#)]
fn test_assert_unique_fails_with_message() {
    let items = vec![1, 2, 2, 4, 4, 4];
    assert_unique!(items, "Dummy message: {}", "xyzzy");
}
