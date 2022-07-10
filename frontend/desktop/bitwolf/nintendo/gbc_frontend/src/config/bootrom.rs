use anyhow::{anyhow, Result};
use std::path::PathBuf;

pub fn load_bootrom() -> Result<[u8; 256]> {
    let path = PathBuf::from("/home/nibble/Downloads/dmg_boot.bin");
    let bootrom = std::fs::read(path.clone())
        .or_else(|_| Err(anyhow!("Unable to read bootrom '{path:?}'")))?;
    if bootrom.len() != 256 {
        return Err(anyhow!("Invalid bootrom size of {}", bootrom.len() as u64));
    }
    let bootrom = {
        let mut arr = [0; 256];
        for i in 0..256 {
            arr[i] = bootrom[i];
        }
        arr
    };
    Ok(bootrom)
}
