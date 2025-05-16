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
    let mut words_with_freq: Vec<(String, u64)> = fs::read_to_string(input_tsv_path)?
        .lines()
        .filter_map(|l| {
            let mut it = l.split('\t');
            Some((it.next()?.to_owned(), it.next()?.parse::<u64>().ok()?))
        })
        .collect();

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
    for (word, _freq) in words_with_freq {
        let rank = word_to_rank.get(&word).copied().unwrap_or(0); // Should always find it
        build.insert(word, rank)?;
    }
    build.finish()?;
    Ok(())
}
