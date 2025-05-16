use std::{env, fs, path::Path};
use fst::MapBuilder;           // only MapBuilder is needed
use anyhow::Result;

/// Converts a word-frequency TSV into a minimal FST
/// whose *value* is the rank (0 = most frequent).
fn main() -> Result<()> {
    let out_dir = env::var("OUT_DIR")?;
    let dict_path = Path::new(&out_dir).join("dict.fst");

    // Read & sort by descending freq
    let mut words: Vec<(String, u64)> = fs::read_to_string("data/words.tsv")?
        .lines()
        .filter_map(|l| {
            let mut it = l.split('\t');
            Some((it.next()?.to_owned(), it.next()?.parse::<u64>().ok()?))
        })
        .collect();

    words.sort_unstable_by(|a, b| b.1.cmp(&a.1));       // high-freq first

    // Build FST map: key = word, value = rank (0..N)
    let wtr = fs::File::create(&dict_path)?;
    let mut build = MapBuilder::new(wtr)?;
    for (rank, (word, _freq)) in words.into_iter().enumerate() {
        build.insert(word, rank as u64)?;
    }
    build.finish()?;

    // Make Cargo rerun if the source list changes
    println!("cargo:rerun-if-changed=data/words.tsv");
    Ok(())
}
