pub mod dictionary;
pub mod suggest;

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn setup_test_dictionary() {
        // Ensure the test dictionary is built and load it.
        // The build script should have created this file in the data directory.
        let test_dict_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("data").join("test_dict.fst");
        if !test_dict_path.exists() {
            // Attempt to run the build script logic directly if the file doesn't exist.
            // This is a fallback and ideally the build script handles this.
            // Note: This is a simplified version and might not perfectly replicate the build script.
            panic!("Test dictionary FST not found at {}. Please ensure build.rs ran correctly.", test_dict_path.display());
        }
        dictionary::load(&test_dict_path).expect("Failed to load test dictionary");
    }

    #[test]
    fn test_empty_buffer() {
        setup_test_dictionary();
        let suggestions = suggest::candidates("");
        assert!(suggestions.is_empty(), "Expected no suggestions for empty buffer");
    }

    #[test]
    fn test_exact_match() {
        setup_test_dictionary();
        let suggestions = suggest::candidates("apple");
        assert!(suggestions.contains(&"apple".to_string()), "Expected 'apple' in suggestions for exact match");
    }

    #[test]
    fn test_one_edit_distance() {
        setup_test_dictionary();
        // 'apply' is one edit away from 'apple' (p -> y)
        let suggestions = suggest::candidates("apply");
        assert!(suggestions.contains(&"apple".to_string()), "Expected 'apple' in suggestions for 'apply'");
    }

    #[test]
    fn test_two_edit_distance() {
        setup_test_dictionary();
        // 'axrle' is two edits away from 'apple' (x -> p, r -> p)
        let suggestions = suggest::candidates("axrle");
        assert!(suggestions.contains(&"apple".to_string()), "Expected 'apple' in suggestions for 'axrle'");
    }

    #[test]
    fn test_no_match_within_max_edits() {
        setup_test_dictionary();
        // 'xyzxyz' should be too far from any word in the test dictionary
        let suggestions = suggest::candidates("xyzxyz");
        assert!(suggestions.is_empty(), "Expected no suggestions for 'xyzxyz'");
    }

    #[test]
    fn test_suggestions_sorted_by_rank() {
        // banana (20), berry (15), apple (10), cherry (8), apricot (5), bandana (1)
        // New ranks with searchkeyX words (all freq 1, so higher rank numbers):
        // banana (20) -> 0
        // berry (15) -> 1
        // apple (10) -> 2
        // cherry (8) -> 3
        // apricot (5) -> 4
        // bandana (1) -> 5
        // searchkeya (1) -> 6 (alphabetically first of freq 1 after bandana)
        // ...and so on for other searchkeyX
        setup_test_dictionary();
        // Input "bana" is 1 edit from "banana" and "bandana".
        // Levenshtein("bana", "banana") = 1 (delete last 'a')
        // Levenshtein("bana", "bandana") = 1 (change 'n' to 'd')
        let mut actual_suggestions = suggest::candidates("bana");

        // Expected: banana (rank 0)
        // FST results are sorted by key. So, actual_suggestions will be ["banana"]
        let mut expected = vec!["banana"];
        expected.sort();
        actual_suggestions.sort();
        
        assert!(expected.iter().all(|item| actual_suggestions.contains(&item.to_string())) && expected.len() == actual_suggestions.len(),
                "Expected suggestions for 'bana' to be {:?}, got {:?}", expected, actual_suggestions);
    }

    #[test]
    fn test_top_k_limit() {
        setup_test_dictionary();
        // With "searchkey" and MAX_EDITS=2, we expect to find:
        // searchkeya (1 edit)
        // searchkeyb (1 edit)
        // searchkeyc (1 edit)
        // searchkeyd (1 edit)
        // searchkeye (1 edit)
        // searchkeyf (1 edit)
        // These are 6 items. TOP_K = 5.
        // FST will return them alphabetically: searchkeya, searchkeyb, searchkeyc, searchkeyd, searchkeye, searchkeyf
        // So, we expect the first 5.
        let suggestions = suggest::candidates("searchkey");
        assert_eq!(suggestions.len(), 5, "Expected suggestions to be limited by TOP_K for input 'searchkey'");

        // Optionally, check the content too
        let expected_slice = ["searchkeya", "searchkeyb", "searchkeyc", "searchkeyd", "searchkeye"];
        assert!(suggestions.iter().all(|s| expected_slice.contains(&s.as_str())),
                "Unexpected suggestions for 'searchkey': {:?}, expected subset of {:?}", suggestions, expected_slice);
        // More strictly check they are exactly these 5
        let suggestions_set: std::collections::HashSet<String> = suggestions.iter().cloned().collect();
        let expected_set: std::collections::HashSet<String> = expected_slice.iter().map(|&s| s.to_string()).collect();
        assert_eq!(suggestions_set, expected_set);
    }
}
