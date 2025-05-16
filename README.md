# Autocorrect

Autocorrect is a lightweight, high-performance Rust library and CLI tool for on-device fuzzy spelling correction. It compiles a simple TSV word-frequency list into a memory-mapped finite-state transducer (FST) and streams Levenshtein-based suggestions in sub-millisecond time.

---

## Features

- **Offline Dictionary Build**: Converts `words.tsv` into a minimal FST during build time using `fst::MapBuilder`.
- **Memory-Mapped Lexicon**: Loads the FST at runtime with `memmap2`, keeping RAM usage minimal.
- **Real-Time Fuzzy Search**: Provides top-K nearest-word suggestions on every keystroke with configurable edit distance.
- **Interactive REPL Demo**: A CLI demo (`main.rs`) that shows suggestions live as you type.
- **Extensible**: Easily bolt on touch-geometry weighting, quantized neural-LM reranking, or custom automata.
- **Tiny Footprint**: Binary size <2 MB, hot RAM <1 MB, latency <1 ms per lookup.

---

## Getting Started

### Prerequisites

- Rust (stable) installed via [rustup](https://rustup.rs)
- A word-frequency TSV (`data/words.tsv`) with lines formatted as `<word>\t<frequency>`

### Project Layout

```
autocorrect/
├─ Cargo.toml
├─ build.rs
├─ data/
│  └─ words.tsv        # word<TAB>frequency list
└─ src/
   ├─ lib.rs           # exposes dictionary & suggest modules
   ├─ dictionary.rs    # FST loader
   ├─ suggest.rs       # candidate generator
   └─ main.rs          # CLI REPL demo
```

### Installation

Clone the repository and build in release mode:

```bash
git clone https://github.com/yourusername/autocorrect.git
cd autocorrect
cargo build --release
```

This will run `build.rs` to generate `dict.fst` from `data/words.tsv` and compile the binary to `target/release/autocorrect`.

## Usage

### CLI Demo

Launch the interactive REPL:

```bash
./target/release/autocorrect
```

Start typing—suggestions appear live:

```
Start typing… (Ctrl-D to quit)
t ▶ ["the", "to", "tea", "too", "ten"]
th ▶ ["the", "they", "then", "that", "thus"]
```

### Library API

Include in your `Cargo.toml`:

```toml
[dependencies]
autocorrect = { path = "../autocorrect" }
```

Load the dictionary and query suggestions:

```rust
use autocorrect::{dictionary, suggest};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    // Load the FST
    dictionary::load(Path::new("path/to/dict.fst"))?;

    // Get suggestions
    let s = suggest::candidates("exampl");
    println!("Suggestions: {:?}", s);
    Ok(())
}
```

## Dictionary Format

The source TSV (`data/words.tsv`) must be in the format:

```text
<word><TAB><frequency>
```

- `<word>`: UTF-8 string, usually lowercase.
- `<frequency>`: integer count or weight.

Lines should be sorted by descending frequency for optimal rank assignment.

## Extending

- **Touch-Geometry**: Implement a custom `fst::Automaton` to weight edits by key proximity.
- **Neural-LM Reranking**: Post-filter the top-K candidates with a quantized Transformer or LSTM for context.

## Contributing

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Commit your changes (`git commit -am 'Add new feature'`).
4. Push to your branch (`git push origin feature/your-feature`).
5. Submit a pull request.

Please ensure all changes include tests and you’ve run `cargo fmt` and `cargo clippy`.

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
