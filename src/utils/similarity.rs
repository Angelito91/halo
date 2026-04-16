// The Halo Programming Language
// Version: 0.2.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! String similarity utilities for error suggestions
//!
//! Provides Levenshtein distance calculation and similar string matching
//! for "Did You Mean" style error suggestions.

use std::cmp::Ordering;

/// Calculate Levenshtein distance between two strings
///
/// The Levenshtein distance is the minimum number of single-character edits
/// (insertions, deletions, substitutions) needed to transform one string into another.
///
/// # Arguments
/// * `s1` - First string
/// * `s2` - Second string
///
/// # Returns
/// The Levenshtein distance (0 = identical, higher = more different)
///
/// # Example
/// ```ignore
/// assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
/// assert_eq!(levenshtein_distance("hello", "hello"), 0);
/// ```
pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();

    // Handle empty strings
    if len1 == 0 {
        return len2;
    }
    if len2 == 0 {
        return len1;
    }

    // Create a matrix for dynamic programming
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];

    // Initialize first row and column
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }

    // Convert to chars for easier comparison
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();

    // Fill the matrix using dynamic programming
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1_chars[i - 1] == s2_chars[j - 1] {
                0
            } else {
                1
            };

            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1, // deletion
                    matrix[i][j - 1] + 1, // insertion
                ),
                matrix[i - 1][j - 1] + cost, // substitution
            );
        }
    }

    matrix[len1][len2]
}

/// A suggestion with its similarity score
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Suggestion {
    pub text: String,
    pub distance: usize,
}

impl Suggestion {
    pub fn new(text: String, distance: usize) -> Self {
        Suggestion { text, distance }
    }
}

impl std::fmt::Display for Suggestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Ord for Suggestion {
    fn cmp(&self, other: &Self) -> Ordering {
        // Sort by distance (ascending), then by length (ascending), then alphabetically
        self.distance
            .cmp(&other.distance)
            .then_with(|| self.text.len().cmp(&other.text.len()))
            .then_with(|| self.text.cmp(&other.text))
    }
}

impl PartialOrd for Suggestion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Find similar identifiers from a list of candidates
///
/// # Arguments
/// * `typo` - The misspelled identifier
/// * `candidates` - List of valid identifiers to compare against
/// * `max_suggestions` - Maximum number of suggestions to return
/// * `max_distance` - Maximum Levenshtein distance to consider
///
/// # Returns
/// Vector of suggestions sorted by distance (closest first)
///
/// # Example
/// ```ignore
/// let candidates = vec!["add", "subtract", "multiply"];
/// let suggestions = find_suggestions("ad", &candidates, 3, 2);
/// assert_eq!(suggestions[0].text, "add");
/// ```
pub fn find_suggestions(
    typo: &str,
    candidates: &[&str],
    max_suggestions: usize,
    max_distance: usize,
) -> Vec<Suggestion> {
    let mut suggestions: Vec<Suggestion> = candidates
        .iter()
        .map(|candidate| {
            let distance = levenshtein_distance(typo, candidate);
            Suggestion::new(candidate.to_string(), distance)
        })
        .filter(|s| s.distance <= max_distance && s.distance > 0)
        .collect();

    suggestions.sort();
    suggestions.truncate(max_suggestions);
    suggestions
}

/// Find similar identifiers with smart distance threshold
///
/// The threshold is adaptive based on the length of the typo:
/// - Strings < 3 chars: max distance = 1
/// - Strings 3-5 chars: max distance = 2
/// - Strings >= 6 chars: max distance = 3
///
/// # Arguments
/// * `typo` - The misspelled identifier
/// * `candidates` - List of valid identifiers to compare against
/// * `max_suggestions` - Maximum number of suggestions to return
///
/// # Returns
/// Vector of suggestions sorted by distance (closest first)
pub fn find_suggestions_smart(
    typo: &str,
    candidates: &[&str],
    max_suggestions: usize,
) -> Vec<Suggestion> {
    let typo_len = typo.len();

    // Adaptive distance threshold based on string length
    let max_distance = if typo_len < 3 {
        1
    } else if typo_len < 6 {
        2
    } else {
        3
    };

    find_suggestions(typo, candidates, max_suggestions, max_distance)
}

/// Find similar identifiers with adaptive threshold
///
/// The threshold allows up to 30% of the string length as distance,
/// with a minimum of 1.
///
/// # Arguments
/// * `typo` - The misspelled identifier
/// * `candidates` - List of valid identifiers to compare against
/// * `max_suggestions` - Maximum number of suggestions to return
///
/// # Returns
/// Vector of suggestions sorted by distance (closest first)
pub fn find_suggestions_adaptive(
    typo: &str,
    candidates: &[&str],
    max_suggestions: usize,
) -> Vec<Suggestion> {
    let typo_len = typo.len() as f64;

    // Adaptive threshold: allow up to 30% of the length as distance
    let max_distance = std::cmp::max(1, (typo_len * 0.3).ceil() as usize);

    find_suggestions(typo, candidates, max_suggestions, max_distance)
}
