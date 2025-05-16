use std::path::Path;
use fst::Map;
use memmap2::Mmap;
use once_cell::sync::OnceCell;
use std::fs::File;

static DICT: OnceCell<Map<Mmap>> = OnceCell::new();

pub fn load(path: &Path) -> anyhow::Result<()> {
    let file = File::open(path)?;
    let mmap = unsafe { Mmap::map(&file)? };
    let map = Map::new(mmap)?;
    DICT.set(map).ok();
    Ok(())
}

pub fn dict() -> &'static Map<Mmap> {
    DICT.get().expect("dictionary not loaded")
}
