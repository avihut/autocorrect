use fst::automaton::Levenshtein;
use fst::{IntoStreamer, Streamer};
use crate::dictionary::dict;

/// Max edit distance we tolerate per keystroke
const MAX_EDITS: u32 = 2;
/// How many suggestions to surface
const TOP_K: usize   = 5;

/// Returns the top-K nearest words for the current buffer.
///
/// *`buffer`* is what the user has typed so far.
pub fn candidates(buffer: &str) -> Vec<String> {
    if buffer.is_empty() {
        return Vec::new();
    }

    // 1. Build an on-the-fly Levenshtein automaton
    let lev = Levenshtein::new(buffer, MAX_EDITS).unwrap();

    // 2. Stream through the FST; items come out ordered by transducer value (=rank)
    let mut stream = dict().search(&lev).into_stream();

    let mut out = Vec::with_capacity(TOP_K);
    while let Some((key, _rank)) = stream.next() {
        out.push(String::from_utf8_lossy(key).to_string());
        if out.len() == TOP_K {
            break;
        }
    }
    out
}
