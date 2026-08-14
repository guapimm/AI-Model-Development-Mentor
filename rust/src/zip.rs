// Minimal ZIP writer (STORE, no compression) - zero dependencies.
use std::fs;
use std::io::{self, Write};
use std::path::Path;

const CRC_TABLE: [u32; 256] = build_crc_table();

const fn build_crc_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    let mut i = 0;
    while i < 256 {
        let mut c = i as u32;
        let mut k = 0;
        while k < 8 {
            c = if c & 1 != 0 { 0xEDB88320 ^ (c >> 1) } else { c >> 1 };
            k += 1;
        }
        table[i] = c;
        i += 1;
    }
    table
}

pub fn crc32(data: &[u8]) -> u32 {
    let mut c: u32 = 0xFFFF_FFFF;
    for &b in data {
        c = CRC_TABLE[((c ^ b as u32) & 0xFF) as usize] ^ (c >> 8);
    }
    c ^ 0xFFFF_FFFF
}

pub fn write_stored_zip(src_dir: &Path, out_zip: &Path) -> io::Result<()> {
    let mut entries: Vec<(String, std::path::PathBuf)> = Vec::new();
    collect_files(src_dir, "", &mut entries)?;
    entries.sort();

    let mut file = fs::File::create(out_zip)?;
    let mut central: Vec<u8> = Vec::new();
    let mut offset: u32 = 0;

    for (name, path) in &entries {
        let data = fs::read(path)?;
        let name_bytes = name.as_bytes();
        let crc = crc32(&data);
        let size = data.len() as u32;

        // Local file header (STORE)
        let mut local = Vec::new();
        local.extend_from_slice(&0x04034b50u32.to_le_bytes());
        local.extend_from_slice(&20u16.to_le_bytes());
        local.extend_from_slice(&0x0800u16.to_le_bytes()); // UTF-8 flag
        local.extend_from_slice(&0u16.to_le_bytes()); // compression: STORE
        local.extend_from_slice(&0u16.to_le_bytes());
        local.extend_from_slice(&0u16.to_le_bytes());
        local.extend_from_slice(&crc.to_le_bytes());
        local.extend_from_slice(&size.to_le_bytes());
        local.extend_from_slice(&size.to_le_bytes());
        local.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        local.extend_from_slice(&0u16.to_le_bytes()); // extra len

        file.write_all(&local)?;
        file.write_all(name_bytes)?;
        file.write_all(&data)?;

        // Central directory entry
        let mut cen = Vec::new();
        cen.extend_from_slice(&0x02014b50u32.to_le_bytes());
        cen.extend_from_slice(&20u16.to_le_bytes()); // version made by
        cen.extend_from_slice(&20u16.to_le_bytes()); // version needed
        cen.extend_from_slice(&0x0800u16.to_le_bytes()); // UTF-8 flag
        cen.extend_from_slice(&0u16.to_le_bytes()); // STORE
        cen.extend_from_slice(&0u16.to_le_bytes());
        cen.extend_from_slice(&0u16.to_le_bytes());
        cen.extend_from_slice(&crc.to_le_bytes());
        cen.extend_from_slice(&size.to_le_bytes());
        cen.extend_from_slice(&size.to_le_bytes());
        cen.extend_from_slice(&(name_bytes.len() as u16).to_le_bytes());
        cen.extend_from_slice(&0u16.to_le_bytes()); // extra len
        cen.extend_from_slice(&0u16.to_le_bytes()); // comment len
        cen.extend_from_slice(&0u16.to_le_bytes()); // disk start
        cen.extend_from_slice(&0u16.to_le_bytes()); // internal attrs
        cen.extend_from_slice(&0u32.to_le_bytes()); // external attrs
        cen.extend_from_slice(&offset.to_le_bytes());
        cen.extend_from_slice(name_bytes);
        central.extend_from_slice(&cen);

        offset += (local.len() as u32) + (name_bytes.len() as u32) + size;
    }

    let central_offset = offset;
    let central_size = central.len() as u32;
    file.write_all(&central)?;

    // End of central directory
    let mut eocd = Vec::new();
    eocd.extend_from_slice(&0x06054b50u32.to_le_bytes());
    eocd.extend_from_slice(&0u16.to_le_bytes());
    eocd.extend_from_slice(&0u16.to_le_bytes());
    eocd.extend_from_slice(&(entries.len() as u16).to_le_bytes());
    eocd.extend_from_slice(&(entries.len() as u16).to_le_bytes());
    eocd.extend_from_slice(&central_size.to_le_bytes());
    eocd.extend_from_slice(&central_offset.to_le_bytes());
    eocd.extend_from_slice(&0u16.to_le_bytes());
    file.write_all(&eocd)?;
    file.flush()?;
    Ok(())
}

fn collect_files(dir: &Path, prefix: &str, out: &mut Vec<(String, std::path::PathBuf)>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();
        let rel = if prefix.is_empty() { name.clone() } else { format!("{}/{}", prefix, name) };
        if path.is_dir() {
            collect_files(&path, &rel, out)?;
        } else {
            out.push((rel, path));
        }
    }
    Ok(())
}
