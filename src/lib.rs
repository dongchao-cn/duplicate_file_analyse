use walkdir::WalkDir;
use std::collections::HashMap;
use std::hash::Hash;
use hex;
use sha1::{Sha1, Digest};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::hash::Hasher;


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

pub fn analyse_duplicated_floder(duplicated_files: HashMap<String, Vec<String>>) -> HashMap<VecKey, u32> {
    let mut result: HashMap<VecKey, u32> = HashMap::new();
    for (_hash, file_vec) in &duplicated_files {
        let mut file_path_vec: Vec<String> = file_vec.iter().map(|f| Path::new(f).parent().map(|p: &Path| p.to_str().unwrap().to_string()).unwrap()).collect();
        file_path_vec.sort();
        let file_path_veckey = VecKey::new(file_path_vec);
        *result.entry(file_path_veckey).or_insert(0) += 1;
    }

    result
}

pub fn get_duplicated_files(all_files: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut all_files_hash: HashMap<String, Vec<String>> = HashMap::new();
    for each in all_files {
        let each_hash = calc_hash(each);
        all_files_hash.entry(each_hash).or_insert_with(Vec::new).push(each.to_string());
    }
    let mut duplicated_files = HashMap::new();
    for (hash, file_vec) in all_files_hash {
        if file_vec.len() >= 2 {
            duplicated_files.insert(hash, file_vec);
        }
    }
    duplicated_files
}

pub fn get_all_files(path: &str) -> Vec<String> {
    let mut result = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let metadata = entry.metadata().unwrap();
            if metadata.len() > 0 {
                let p = entry.path().to_str().unwrap().to_string();
                // println!("{}", p.display());
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
    let mut buffer = [0; READ_BLOCK_SIZE];
    // let mut buffer = vec![0; READ_BLOCK_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer).unwrap();

        // dbg!(bytes_read);

        if bytes_read == 0 {
            break;
        }

        hasher.update(&buffer);
    }

    let result = hasher.finalize();

    // println!("{}", hex::encode(result));
    hex::encode(result)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_calc_hash() {
        let p = "/home/dongchao/duplicate_file_analyse/Cargo.toml";
        println!("{}", calc_hash(p))
    }
}
