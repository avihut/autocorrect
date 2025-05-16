use std::{env, fs, path::Path};
use fst::MapBuilder;           // only MapBuilder is needed
use anyhow::Result;

/// Converts a word-frequency TSV into a minimal FST
/// whose *value* is the rank (0 = most frequent).
fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let dict_path = Path::new(&out_dir).join("dict.fst");

    // Build main dictionary
    build_dictionary("data/words.tsv", &dict_path)?;

    // Build test dictionary
    // The test dictionary will be placed directly in the data directory
    // as it's needed at test time, not just build time like the main dictionary.
    let test_dict_path = Path::new("data").join("test_dict.fst");
    build_dictionary("data/test_words.tsv", &test_dict_path)?;

    // Make Cargo rerun if the source list changes
    println!("cargo:rerun-if-changed=data/words.tsv");
    println!("cargo:rerun-if-changed=data/test_words.tsv"); // Rerun if test words change
    Ok(())
}

fn build_dictionary(input_tsv_path: &str, output_fst_path: &Path) -> Result<()> {
    // Read words and their frequencies
    let file_content = fs::read_to_string(input_tsv_path)?;
    println!("cargo:warning=First 500 chars of {}: {}", input_tsv_path, file_content.chars().take(500).collect::<String>());

    let mut words_with_freq: Vec<(String, u64)> = Vec::new();
    let mut line_number = 0;
    let mut parse_errors = 0;

    for line in file_content.lines() {
        line_number += 1;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // Debug the first few problematic lines
        if parse_errors < 5 {
            println!("cargo:warning=Processing line {}: '{}'", line_number, line);
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 2 {
            if parse_errors < 5 {
                println!("cargo:warning=Line {} has {} parts instead of 2: '{}'", line_number, parts.len(), line);
            }
            parse_errors += 1;
            continue;
        }

        let word = parts[0].trim();
        if word.is_empty() {
            if parse_errors < 5 {
                println!("cargo:warning=Line {} has empty word: '{}'", line_number, line);
            }
            parse_errors += 1;
            continue;
        }

        match parts[1].trim().parse::<u64>() {
            Ok(freq) => {
                words_with_freq.push((word.to_string(), freq));
            }
            Err(e) => {
                if parse_errors < 5 {
                    println!("cargo:warning=Line {} has invalid frequency '{}': {}", line_number, parts[1], e);
                }
                parse_errors += 1;
            }
        }
    }

    println!("cargo:warning=Parsed {} words with frequencies from {} ({} parse errors)", 
             words_with_freq.len(), input_tsv_path, parse_errors);

    if words_with_freq.is_empty() {
        println!("cargo:warning=No words parsed from {}. FST will be empty.", input_tsv_path);
        // Still proceed to create an empty FST to avoid breaking the build script further down if it expects a file
    }

    // Create a version sorted by frequency to determine ranks
    let mut ranked_words = words_with_freq.clone();
    ranked_words.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0))); // Sort by freq (desc), then word (asc) for stable rank

    // Create a map of word to rank
    let word_to_rank: std::collections::HashMap<String, u64> = ranked_words
        .into_iter()
        .enumerate()
        .map(|(rank, (word, _freq))| (word, rank as u64))
        .collect();

    // Sort the original list by word for FST insertion
    words_with_freq.sort_unstable_by(|a, b| a.0.cmp(&b.0));

    // Build FST map: key = word (alphabetical), value = rank (from frequency)
    let wtr = fs::File::create(output_fst_path)?;
    let mut build = MapBuilder::new(wtr)?;
    let mut words_inserted_count = 0;
    for (word, _freq) in &words_with_freq {
        if let Some(rank) = word_to_rank.get(word).copied() {
            build.insert(word, rank)?;
            words_inserted_count += 1;
        } else {
            println!("cargo:warning=Word '{}' not found in rank map. Skipping insertion.", word);
        }
    }
    println!("cargo:warning=Inserted {} words into FST for {}", words_inserted_count, input_tsv_path);
    build.finish()?;
    Ok(())
}
