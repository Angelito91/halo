// The Halo Programming Language
// Version: 0.3.0
// Author: Angel A. Portuondo H.
// License: MPL 2.0
// SPDX-License-Identifier: MPL-2.0

//! Tests for string similarity and suggestion finding algorithms

use halo::diagnostics::suggestions::{
    find_suggestions, find_suggestions_smart, levenshtein_distance, Suggestion,
};

#[test]
fn test_exact_match() {
    assert_eq!(levenshtein_distance("hello", "hello"), 0);
}

#[test]
fn test_single_substitution() {
    assert_eq!(levenshtein_distance("hello", "hallo"), 1);
}

#[test]
fn test_single_insertion() {
    assert_eq!(levenshtein_distance("hello", "helo"), 1);
}

#[test]
fn test_single_deletion() {
    assert_eq!(levenshtein_distance("hello", "helloo"), 1);
}

#[test]
fn test_multiple_edits() {
    assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
}

#[test]
fn test_completely_different() {
    assert_eq!(levenshtein_distance("abc", "xyz"), 3);
}

#[test]
fn test_empty_string_from() {
    assert_eq!(levenshtein_distance("", "hello"), 5);
}

#[test]
fn test_empty_string_to() {
    assert_eq!(levenshtein_distance("hello", ""), 5);
}

#[test]
fn test_both_empty() {
    assert_eq!(levenshtein_distance("", ""), 0);
}

#[test]
fn test_case_sensitive() {
    assert!(levenshtein_distance("Hello", "hello") > 0);
}

#[test]
fn test_find_suggestions() {
    let candidates = vec!["count", "counter", "mount", "print"];
    let suggestions = find_suggestions("cunt", &candidates, 3, 2);

    assert!(!suggestions.is_empty());
    assert_eq!(suggestions[0].text, "count");
}

#[test]
fn test_find_suggestions_smart() {
    let candidates = vec!["add", "subtract", "multiply", "divide"];
    let suggestions = find_suggestions_smart("ad", &candidates, 2);

    assert!(!suggestions.is_empty());
    assert_eq!(suggestions[0].text, "add");
}

#[test]
fn test_suggestion_ordering() {
    let s1 = Suggestion::new("add".to_string(), 1);
    let s2 = Suggestion::new("subtract".to_string(), 2);

    assert!(s1 < s2);
}

#[test]
fn test_suggestion_ordering_same_distance() {
    let s1 = Suggestion::new("abc".to_string(), 1);
    let s2 = Suggestion::new("abcd".to_string(), 1);

    assert!(s1 < s2); // shorter comes first
}

#[test]
fn test_no_exact_matches_returned() {
    let candidates = vec!["add", "subtract"];
    let suggestions = find_suggestions("add", &candidates, 3, 2);

    // Should not include exact matches (distance 0)
    assert!(suggestions.iter().all(|s| s.distance > 0));
}

#[test]
fn test_common_typo_patterns() {
    // Adjacent transposition
    assert!(levenshtein_distance("recieve", "receive") <= 2);

    // Common vowel mistakes
    assert!(levenshtein_distance("color", "colour") == 1);

    // Double letter
    assert!(levenshtein_distance("programming", "programing") == 1);
}

#[test]
fn test_suggestions_threshold() {
    let candidates = vec!["variable", "function", "print", "return"];
    let suggestions = find_suggestions("vriable", &candidates, 2, 2);

    // Should find 'variable' as it's very close
    assert!(suggestions.iter().any(|s| s.text == "variable"));
}

#[test]
fn test_suggestions_max_count() {
    let candidates = vec!["a", "aa", "aaa", "aaaa", "aaaaa"];
    let suggestions = find_suggestions_smart("a", &candidates, 2);

    // Should return at most 2 suggestions
    assert!(suggestions.len() <= 2);
}

#[test]
fn test_suggestions_sort_by_distance() {
    let candidates = vec!["hello", "helo", "hole", "world"];
    let suggestions = find_suggestions("helo", &candidates, 3, 3);

    // Should be sorted by distance (closest first)
    for i in 0..suggestions.len() - 1 {
        assert!(suggestions[i].distance <= suggestions[i + 1].distance);
    }
}

#[test]
fn test_long_strings_similarity() {
    let s1 = "the quick brown fox jumps over the lazy dog";
    let s2 = "the quick brown fox jumps over the lazy cat";

    let distance = levenshtein_distance(s1, s2);
    assert!(distance < 5); // Should be similar overall
}

#[test]
fn test_single_character_strings() {
    assert_eq!(levenshtein_distance("a", "a"), 0);
    assert_eq!(levenshtein_distance("a", "b"), 1);
    assert_eq!(levenshtein_distance("a", ""), 1);
}

#[test]
fn test_numeric_string_suggestions() {
    let candidates = vec!["add", "sub", "mul", "div"];
    let suggestions = find_suggestions_smart("ad", &candidates, 2);

    assert!(!suggestions.is_empty());
    assert!(suggestions[0].text == "add" || suggestions[0].text == "sub");
}
