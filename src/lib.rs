use walkdir::WalkDir;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use dashmap::DashMap;
use hex;
use sha1::{Sha1, Digest};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::hash::Hasher;
use log::info;
use rayon::prelude::*;


#[derive(Debug)]
pub struct VecKey {
    key: Vec<String>
}
impl PartialEq for VecKey {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for VecKey {}
impl Hash for VecKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}
impl VecKey {
    pub fn new(v: Vec<String>) -> Self {
        VecKey {key: v}
    }
}

pub fn analyse_duplicated_floder(duplicated_files: HashMap<String, Vec<String>>) -> Vec<(VecKey, (u32, Vec<String>))> {
    let mut result: HashMap<VecKey, (u32, Vec<String>)> = HashMap::new();
    for (hash, file_vec) in &duplicated_files {
        let mut file_path_vec: Vec<String> = file_vec.iter()
            .map(|f| Path::new(f).parent().map(|p: &Path| p.to_str().unwrap().to_string()).unwrap())
            .collect();
        file_path_vec.sort();
        let file_path_veckey = VecKey::new(file_path_vec);
        let entry = result.entry(file_path_veckey).or_insert((0, Vec::new()));
        entry.0 += 1;
        entry.1.push(hash.clone());
    }
    let mut result_vec: Vec<_> = result.into_iter().collect();
    result_vec.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    result_vec
}

pub fn get_duplicated_files(all_files: &Vec<String>) -> HashMap<String, Vec<String>> {
    let cnt = Arc::new(AtomicU32::new(0));
    let all_files_hash = Arc::new(DashMap::new());
    all_files.par_iter().for_each(|each| {
        let each_hash = calc_hash(each);
        all_files_hash.entry(each_hash).or_insert_with(Vec::new).push(each);
        cnt.fetch_add(1, Ordering::SeqCst);
        info!("{:?}/{}", cnt, all_files.len());
    });

    let all_files_hash = all_files_hash.clone();
    let mut duplicated_files = HashMap::new();
    for kv in all_files_hash.iter() {
        let (hash, file_vec) = kv.pair();
        if file_vec.len() >= 2 {
            duplicated_files.insert(hash.clone(), file_vec.iter().cloned().cloned().collect());
        }
    }

    duplicated_files
}

pub fn get_all_files(path: &str) -> Vec<String> {
    let mut result = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() && entry.path().to_str().map_or(false, |s| !s.contains("@eaDir")) {
            let metadata = entry.metadata().unwrap();
            if metadata.len() > 0 {
                let p = entry.path().to_str().unwrap().to_string();
                // debug!("{}", p.display());
                result.push(p);
            }
        }
    }
    result
}

const READ_BLOCK_SIZE: usize = 4*1024*1024;

fn calc_hash(file_name: &str) -> String {
    let mut hasher = Sha1::new();

    let f = File::open(file_name).unwrap();
    let mut reader = BufReader::new(f);
    // let mut buffer = [0; READ_BLOCK_SIZE];
    let mut buffer = vec![0; READ_BLOCK_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer).unwrap();

        // dbg!(bytes_read);

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();

    // debug!("{}", hex::encode(result));
    hex::encode(result)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_calc_hash() {
        let p = "/home/dongchao/duplicate_file_analyse/Cargo.toml";
        info!("{}", calc_hash(p))
    }
}
