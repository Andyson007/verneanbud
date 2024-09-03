//! Imports all modules
#![deny(
    missing_docs,
    missing_abi,
    missing_fragment_specifier,
    missing_debug_implementations
)]
#![warn(clippy::pedantic, clippy::nursery)]

use core::fmt::Debug;

pub mod app;
pub mod errors;
pub mod ui;

#[must_use]
/// Returns an ordered list how alike it is to
/// the search query
pub fn query<T>(iter: Vec<T>, query: &str) -> Vec<(usize, T)>
where
    T: Score + Clone + Debug,
{
    let mut indexed = iter
        .into_iter()
        .enumerate()
        .filter_map(|x| Some((x.clone(), x.1.score(query)?)))
        .collect::<Vec<_>>();
    indexed.sort_by_key(|x| x.1);
    indexed.into_iter().map(|x| x.0).collect()
}

/// Implements a scoring trait used for ordering the search items
pub trait Score {
    /// The scoring function it should return None if the
    /// search item shouldn't be included in the final list
    fn score(&self, query: &str) -> Option<i64>;
}

// FIXME: This is horrible code for judging the score of a given word
// compared to its query
impl Score for String {
    fn score(&self, query: &str) -> Option<i64> {
        if self.contains(query) {
            i64::try_from(self.len()).ok()
        } else {
            None
        }
    }
}
