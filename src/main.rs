use std::io::{self, Write};
use autocorrect::{dictionary, suggest};

fn main() -> anyhow::Result<()> {
    // Locate dict built by build.rs
    let dict_path = std::path::Path::new(env!("OUT_DIR")).join("dict.fst");
    dictionary::load(&dict_path)?;

    println!("Start typing… (Ctrl-D to quit)");
    let mut buf = String::new();

    loop {
        // Read *one* byte so we emulate keystrokes
        let mut byte = [0u8];
        if io::stdin().read_exact(&mut byte).is_err() { break; }
        if byte[0] == b'\n' || byte[0] == b' ' {          // word boundary
            buf.clear();
            continue;
        }
        buf.push(byte[0] as char);

        // Fetch suggestions
        let suggestions = suggest::candidates(&buf);
        print!("\r{} ▶ {:?}", buf, suggestions);
        io::stdout().flush().unwrap();
    }
    Ok(())
}
