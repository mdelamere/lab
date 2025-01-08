use sha2::{Sha256, Digest};
use std::{fs, io};
use std::collections::HashMap;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct FileHashes {
    hashes: HashMap<String, String>,
}

pub fn compute_hash(file_path: &Path) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut hasher = Sha256::new();
    io::copy(&mut file, &mut hasher)?;
    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}

pub fn hash_directory(dir: &Path) -> io::Result<HashMap<String, String>> {
    let mut file_hashes = HashMap::new();

    for entry in walkdir::WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let hash = compute_hash(file_path)?;
            file_hashes.insert(file_path.to_string_lossy().to_string(), hash);
        }
    }

    Ok(file_hashes)
}

pub fn save_hashes(file_hashes: &HashMap<String, String>, output_file: &Path) -> io::Result<()> {
    let data = FileHashes { hashes: file_hashes.clone() };
    let json = serde_json::to_string_pretty(&data)?;
    fs::write(output_file, json)?;
    Ok(())
}

pub fn load_hashes(input_file: &Path) -> io::Result<HashMap<String, String>> {
    let data = fs::read_to_string(input_file)?;
    let file_hashes: FileHashes = serde_json::from_str(&data)?;
    Ok(file_hashes.hashes)
}
