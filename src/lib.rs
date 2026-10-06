//! A library providing macros and structures for testing set equality and
//! uniqueness.

use std::hash::Hash;

use linked_hash_map::LinkedHashMap;
use linked_hash_set::LinkedHashSet;
use similar::{Algorithm, DiffOp, capture_diff_slices};

/// A structure representing the difference between two sets. It contains
/// elements only in the left set, only in the right set, and in both sets.
///
/// Ordering of the initial inputs is preserved as much as possible in the
/// resulting vectors.
#[derive(Debug)]
pub struct SetDifference<T> {
    /// Elements only in the left set.
    pub left: Vec<T>,
    /// Elements only in the right set.
    pub right: Vec<T>,
    /// Elements in both sets.
    pub both: Vec<T>,
}

impl<T: Eq + Hash + Clone> SetDifference<T> {
    /// Creates a new `SetDifference` from left and right collections of items.
    ///
    /// The items are treated as sets, meaning duplicates are ignored and only
    /// unique elements are considered.  Ordering of the initial inputs is
    /// preserved in the resulting vectors, but the ordering does not otherwise
    /// affect the output.
    pub fn new(left: impl IntoIterator<Item = T>, right: impl IntoIterator<Item = T>) -> Self {
        let mut left: LinkedHashSet<T> = left.into_iter().collect();
        let mut right: LinkedHashSet<T> = right.into_iter().collect();
        let both: LinkedHashSet<T> = left.intersection(&right).cloned().collect();
        both.iter().for_each(|item| {
            left.remove(item);
            right.remove(item);
        });
        Self {
            left: left.into_iter().collect(),
            right: right.into_iter().collect(),
            both: both.into_iter().collect(),
        }
    }
}

#[derive(Debug)]
pub struct MapDifference<K, V> {
    /// Elements only in the left map.
    pub left: Vec<(K, V)>,
    /// Elements only in the right map.
    pub right: Vec<(K, V)>,
    /// Elements with the same key but different values in the two maps.
    pub different: Vec<(K, V, V)>,
    /// Elements in both maps with the same value.
    pub same: Vec<(K, V)>,
}

impl<K: Eq + Hash + Clone, V: Eq + Clone> MapDifference<K, V> {
    /// Creates a new `MapDifference` from left and right collections of items.
    ///
    /// The items are treated as as key-value pairs, meaning the key is used to
    /// determine uniqueness and differences between the left and right
    /// collections.
    pub fn new(
        left: impl IntoIterator<Item = (K, V)>,
        right: impl IntoIterator<Item = (K, V)>,
    ) -> Self {
        let mut left: LinkedHashMap<K, V> = left.into_iter().collect();
        let mut right: LinkedHashMap<K, V> = right.into_iter().collect();
        let mut different = Vec::new();
        let mut same = Vec::new();

        for (k, lv) in left.clone() {
            if let Some(rv) = right.remove(&k) {
                left.remove(&k);
                if lv == rv {
                    same.push((k, lv));
                } else {
                    different.push((k, lv, rv));
                }
            }
        }

        Self {
            left: left.into_iter().collect(),
            right: right.into_iter().collect(),
            different,
            same,
        }
    }
}

/// A structure representing the uniqueness of items in a collection. It
/// contains duplicates along with their counts, and a separate vec of unique
/// items.
///
/// Ordering of the initial inputs is preserved in the resulting vectors as much
/// as possible.
#[derive(Debug)]
pub struct Uniqueness<T> {
    pub duplicates: Vec<(T, usize)>,
    pub unique: Vec<T>,
}

impl<T: Eq + Hash + Clone> Uniqueness<T> {
    /// Creates a new `Uniqueness` from a collection of items.
    pub fn new(items: impl IntoIterator<Item = T>) -> Self {
        let mut unique = LinkedHashSet::new();
        let mut duplicates = LinkedHashMap::new();
        for item in items.into_iter() {
            if !unique.insert(item.clone()) {
                *duplicates.entry(item).or_insert(1) += 1;
            }
        }
        for duplicate in duplicates.keys() {
            unique.remove(duplicate);
        }
        Self {
            duplicates: duplicates.into_iter().collect::<Vec<_>>(),
            unique: unique.into_iter().collect(),
        }
    }
}

#[derive(Debug)]
pub enum SeqMembership<T> {
    Both(Vec<T>),
    Left(Vec<T>),
    Right(Vec<T>),
}

#[derive(Debug)]
pub struct SeqDifference<T>(pub Vec<SeqMembership<T>>);

impl<T: Eq + Hash + Clone> SeqDifference<T> {
    pub fn new(left: impl IntoIterator<Item = T>, right: impl IntoIterator<Item = T>) -> Self {
        let left: Vec<T> = left.into_iter().collect();
        let right: Vec<T> = right.into_iter().collect();
        let ops = capture_diff_slices(Algorithm::Patience, &left, &right);
        let mut result = SeqDifference(Vec::new());
        for op in dbg!(ops) {
            match op {
                DiffOp::Equal { old_index, len, .. } => {
                    result.0.push(SeqMembership::Both(
                        left[old_index..old_index + len].to_vec(),
                    ));
                }
                DiffOp::Delete {
                    old_len, old_index, ..
                } => {
                    result.0.push(SeqMembership::Left(
                        left[old_index..old_index + old_len].to_vec(),
                    ));
                }
                DiffOp::Insert {
                    new_index, new_len, ..
                } => {
                    result.0.push(SeqMembership::Right(
                        right[new_index..new_index + new_len].to_vec(),
                    ));
                }
                DiffOp::Replace {
                    old_index,
                    old_len,
                    new_index,
                    new_len,
                } => {
                    result.0.push(SeqMembership::Left(
                        left[old_index..old_index + old_len].to_vec(),
                    ));
                    result.0.push(SeqMembership::Right(
                        right[new_index..new_index + new_len].to_vec(),
                    ));
                }
            }
        }
        result
    }
}

/// Asserts that two iterable expressions yield the same set of elements,
/// ignoring order and duplicates.
///
/// On failure, this macro will panic with a message showing the elements
/// that are only in the left collection, only in the right collection, and
/// those present in both.
#[macro_export]
macro_rules! assert_eq_sets {
    ($left:expr, $right:expr $(, $format_arg:expr)* $(,)?) => {{
        let diff = $crate::SetDifference::new($left, $right);
        assert!(
            diff.left.is_empty() && diff.right.is_empty(),
            r#"Sets are not equal{}
 left only: {:?},
right only: {:?},
   in both: {:?}"#,
            $crate::maybe_format!($($format_arg),*),
            diff.left,
            diff.right,
            diff.both
        );

    }};
}

/// Asserts that two iterable expressions yield the same set of elements,
/// ignoring order and duplicates.
///
/// On failure, this macro will panic with a message showing the elements
/// that are only in the left collection, only in the right collection, and
/// those present in both.
#[macro_export]
macro_rules! assert_eq_maps {
    ($left:expr, $right:expr $(, $format_arg:expr)* $(,)?) => {{
        let diff = $crate::MapDifference::new($left, $right);
        assert!(
            diff.left.is_empty() && diff.right.is_empty() && diff.different.is_empty(),
            r#"Maps are not equal{}
       left only: {:?},
      right only: {:?},
different values: {:?},
     same values: {:?}"#,
            $crate::maybe_format!($($format_arg),*),
            diff.left,
            diff.right,
            diff.different,
            diff.both
        );

    }};
}

/// Asserts that all items in the given expression are unique.
///
/// On failure, this macro will panic with a message showing the duplicate items
/// along with their counts, and the unique items.
#[macro_export]
macro_rules! assert_unique {
    ($expr:expr $(, $format_arg:expr)* $(,)?) => {{
        let uniqueness = $crate::Uniqueness::new($expr);
        assert!(
            uniqueness.duplicates.is_empty(),
            r#"Items are not unique{}
duplicates: {:?},
    unique: {:?}"#,
            $crate::maybe_format!($($format_arg),*),
            uniqueness.duplicates,
            uniqueness.unique
        );
    }};
}

#[macro_export]
macro_rules! assert_eq_seqs {
    ($left:expr, $right:expr $(, $format_arg:expr)* $(,)?) => {{
        let diff = $crate::SeqDifference::new($left, $right);
        assert!(
            matches!(diff.0.as_slice(), [$crate::SeqMembership::Both(_)]),
            r#"Sequences are not equal{}
diff: {:?}"#,
            $crate::maybe_format!($($format_arg),*),
            diff.0
        );
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! maybe_format {
    ($format:literal $(, $format_arg:expr)*) => {
        format!(concat!(": ", $format) $(, $format_arg)*)
    };
    () => {
        ""
    };
}
