use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Debug)]
pub struct SetDifference<T> {
    pub left: HashSet<T>,
    pub right: HashSet<T>,
    pub both: HashSet<T>,
}

impl<T: Eq + Hash + Clone> SetDifference<T> {
    pub fn new(left: impl IntoIterator<Item = T>, right: impl IntoIterator<Item = T>) -> Self {
        let mut left: HashSet<T> = left.into_iter().collect();
        let mut right: HashSet<T> = right.into_iter().collect();
        let both: HashSet<T> = left.intersection(&right).cloned().collect();
        both.iter().for_each(|item| {
            left.remove(item);
            right.remove(item);
        });
        Self { left, right, both }
    }
}

#[macro_export]
macro_rules! assert_eq_sets {
    ($left:expr, $right:expr $(, $format_arg:expr)* $(,)?) => {{
        let diff = $crate::SetDifference::new($left, $right);
        assert!(
            diff.left.is_empty() && diff.right.is_empty(),
            r#"Sets are not equal:{}
 left only = {:?},
right only = {:?},
   in both = {:?}"#,
            $crate::maybe_format!($($format_arg),*),
            diff.left,
            diff.right,
            diff.both
        );

    }};
}

#[derive(Debug)]
pub struct Uniqueness<T> {
    pub duplicates: Vec<(usize, T)>,
    pub unique: HashSet<T>,
}

impl<T: Eq + Hash + Clone> Uniqueness<T> {
    pub fn new(items: impl IntoIterator<Item = T>) -> Self {
        let mut unique = HashSet::new();
        let mut duplicates = HashMap::new();
        for item in items.into_iter() {
            if !unique.insert(item.clone()) {
                *duplicates.entry(item).or_insert(1) += 1;
            }
        }
        for duplicate in duplicates.keys() {
            unique.remove(duplicate);
        }
        let mut duplicates = duplicates
            .into_iter()
            .map(|(item, count)| (count, item))
            .collect::<Vec<_>>();
        duplicates.sort_by_key(|a| std::cmp::Reverse(a.0));
        Self { duplicates, unique }
    }
}

#[macro_export]
macro_rules! assert_unique {
    ($expr:expr $(, $format_arg:expr)* $(,)?) => {{
        let uniqueness = $crate::Uniqueness::new($expr);
        assert!(
            uniqueness.duplicates.is_empty(),
            r#"Items are not unique:{}
duplicates = {:?},
    unique = {:?}"#,
            $crate::maybe_format!($($format_arg),*),
            uniqueness.duplicates,
            uniqueness.unique
        );
    }};
}

#[macro_export]
macro_rules! maybe_format {
    ($format:literal $(, $format_arg:expr)*) => {
        format!(concat!(" ", $format) $(, $format_arg)*)
    };
    () => {
        ""
    };
}
