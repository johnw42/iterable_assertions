use set_assertions::{assert_eq_maps, assert_eq_seqs, assert_eq_sets, assert_unique};

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
#[should_panic(expected = r#"Sets are not equal
 left only: [4],
right only: [5],
   in both: [1, 2, 3]"#)]
fn test_assert_eq_sets_fails() {
    let a = vec![1, 2, 3, 4];
    let b = vec![1, 2, 3, 5];
    assert_eq_sets!(a, b);
}

#[test]
#[should_panic(expected = r#"Sets are not equal: [1, 2, 3, 4] != [1, 2, 3, 5]
 left only: [4],
right only: [5],
   in both: [1, 2, 3]"#)]
fn test_assert_eq_sets_fails_with_message() {
    let a = vec![1, 2, 3, 4];
    let b = vec![1, 2, 3, 5];
    assert_eq_sets!(&a, &b, "{:?} != {:?}", a, b);
}

#[test]
fn test_assert_eq_maps() {
    let a = [("a", "A"), ("b", "B")];
    let b = [("b", "B"), ("a", "A")];
    assert_eq_maps!(a, b);
}

#[test]
fn test_assert_eq_maps_with_refs() {
    let a = [("a", "A"), ("b", "B")];
    let b = [("b", "B"), ("a", "A")];

    // Passing references directly doesn't work because we need pairs, not
    // references to pairs, but if we convert to pairs of references, it works.
    fn ref_to_pair<K, V>((k, v): &(K, V)) -> (&K, &V) {
        (k, v)
    }

    assert_eq_maps!(a.iter().map(ref_to_pair), b.iter().map(ref_to_pair));
}

#[test]
#[should_panic(expected = r#"Maps are not equal
       left only: [("d", 4)],
      right only: [("e", 5)],
different values: [("c", 3, 4)],
     same values: [("a", 1), ("b", 2)]"#)]
fn test_assert_eq_maps_fails() {
    let a = [("a", 1), ("b", 2), ("c", 3), ("d", 4)];
    let b = [("a", 1), ("b", 2), ("c", 4), ("e", 5)];
    assert_eq_maps!(a, b);
}

#[test]
#[should_panic(expected = r#"Maps are not equal: xyzzy
       left only: [("d", 4)],
      right only: [("e", 5)],
different values: [("c", 3, 4)],
     same values: [("a", 1), ("b", 2)]"#)]
fn test_assert_eq_maps_fails_with_message() {
    let a = [("a", 1), ("b", 2), ("c", 3), ("d", 4)];
    let b = [("a", 1), ("b", 2), ("c", 4), ("e", 5)];
    assert_eq_maps!(a, b, "xyzzy");
}

#[test]
#[should_panic(expected = r#"Duplicate key found in left map: "a""#)]
fn test_assert_eq_map_fails_on_duplicates_left() {
    let a = [("a", 1), ("a", 1)];
    let b = [];
    assert_eq_maps!(a, b);
}

#[test]
#[should_panic(expected = r#"Duplicate key found in right map: "a""#)]
fn test_assert_eq_map_fails_on_duplicates_right() {
    let a = [];
    let b = [("a", 1), ("a", 1)];
    assert_eq_maps!(a, b);
}

#[test]
fn test_assert_unique() {
    let items = vec![1, 2, 3];
    assert_unique!(items);
}

#[test]
#[should_panic(expected = r#"Items are not unique
duplicates: [(2, 2), (4, 3)],
    unique: [1, 3]"#)]
fn test_assert_unique_fails() {
    let items = vec![1, 2, 2, 3, 4, 4, 4];
    assert_unique!(items);
}

#[test]
#[should_panic(expected = r#"Items are not unique: Dummy message: xyzzy
duplicates: [(2, 2), (4, 3)],
    unique: [1, 3]"#)]
fn test_assert_unique_fails_with_message() {
    let items = vec![1, 2, 2, 3, 4, 4, 4];
    assert_unique!(items, "Dummy message: {}", "xyzzy");
}

#[test]
fn test_assert_eq_seqs() {
    assert_eq_seqs!([1, 2, 3], [1, 2, 3]);
}

#[test]
#[should_panic(expected = r#"Sequences are not equal
diff: [Both([1, 2, 3]), Right([4, 5])]"#)]
fn test_assert_eq_seqs_insertion() {
    assert_eq_seqs!([1, 2, 3], [1, 2, 3, 4, 5]);
}

#[test]
#[should_panic(expected = r#"Sequences are not equal
diff: [Both([1, 2, 3]), Left([4, 5])]"#)]
fn test_assert_eq_seqs_deletion() {
    assert_eq_seqs!([1, 2, 3, 4, 5], [1, 2, 3]);
}

#[test]
#[should_panic(expected = r#"Sequences are not equal
diff: [Both([1, 2]), Left([3]), Right([-3]), Both([4, 5])]"#)]
fn test_assert_eq_seqs_replacement() {
    assert_eq_seqs!([1, 2, 3, 4, 5], [1, 2, -3, 4, 5]);
}
