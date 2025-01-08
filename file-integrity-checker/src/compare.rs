use std::collections::HashMap;

pub fn compare_hashes(
    current: &HashMap<String, String>,
    baseline: &HashMap<String, String>,
) -> (Vec<String>, Vec<String>, Vec<String>) {
    let mut modified = vec![];
    let mut new_files = vec![];
    let mut deleted = vec![];

    for (path, hash) in current {
        match baseline.get(path) {
            Some(baseline_hash) if baseline_hash == hash => (),
            Some(_) => modified.push(path.clone()),
            None => new_files.push(path.clone()),
        }
    }

    for path in baseline.keys() {
        if !current.contains_key(path) {
            deleted.push(path.clone());
        }
    }

    (modified, new_files, deleted)
}
