use std::path::Path;
use fst::Map;
use memmap2::Mmap;
use once_cell::sync::OnceCell;

static DICT: OnceCell<Map<Mmap>> = OnceCell::new();

pub fn load(path: &Path) -> anyhow::Result<()> {
    let map = unsafe { Map::from_path(path)? }; // mmap
    DICT.set(map).ok();
    Ok(())
}

pub fn dict() -> &'static Map<Mmap> {
    DICT.get().expect("dictionary not loaded")
}
