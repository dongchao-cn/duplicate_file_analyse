use walkdir::WalkDir;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use dashmap::DashMap;
use hex;
use sha1::{Sha1, Digest};
use std::fs::File;
use std::io::{stdin, BufReader, Read};
use std::path::Path;
use std::hash::Hasher;
use log::{info, warn};
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

pub fn analyse_duplicated_floder(duplicated_files: HashMap<String, Vec<String>>) -> Vec<(VecKey, (u32, HashMap<String, Vec<(String, String)>>))> {
    let mut result: HashMap<VecKey, (u32, HashMap<String, Vec<(String, String)>>)> = HashMap::new();
    for (hash, file_vec) in &duplicated_files {
        // 获取文件修改时间
        let mut modified_time_vec = Vec::new();
        for file in file_vec {
            let metadata = std::fs::metadata(file).unwrap();
            let modified_time = metadata.modified().unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
            let modified_time = chrono::DateTime::from_timestamp(modified_time as i64, 0).unwrap().format("%Y-%m-%d %H:%M:%S").to_string();
            // info!("File: {}, Modified Time: {:?}", file, modified_time);
            modified_time_vec.push((file.clone(), modified_time))
        }
        modified_time_vec.sort_by(|x, y| x.0.cmp(&y.0));

        let mut file_path_vec: Vec<String> = file_vec.iter()
            .map(|f| Path::new(f).parent().map(|p: &Path| p.to_str().unwrap().to_string()).unwrap())
            .collect();
        file_path_vec.sort();
        let file_path_veckey = VecKey::new(file_path_vec);
        let entry = result.entry(file_path_veckey).or_insert((0, HashMap::new()));
        entry.0 += 1;
        entry.1.entry(hash.clone()).insert_entry(modified_time_vec);
    }
    let mut result_vec: Vec<_> = result.into_iter().collect();
    result_vec.sort_by(|a, b| b.1.0.cmp(&a.1.0));
    result_vec
}

pub fn gen_delete_cmd(analyse_result: Vec<(VecKey, (u32, HashMap<String, Vec<(String, String)>>))>) {
    for item in &analyse_result {
        warn!("select: {:#?}", item);
        for (index, path) in item.0.key.iter().enumerate() {
            warn!("remain: ({}) {:#?}", index, path);
        }
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let input_num: usize = input.trim().parse().unwrap();
        
        for (hash, path) in &item.1.1 {
            for (index, (file, _modified_time)) in path.iter().enumerate() {
                if index != input_num {
                    warn!("rm -rf \"{}\"; # {}", file, hash);
                    warn!(target: "app::del_file", "rm -rf \"{}\"; # {}", file, hash);
                }
            }
        }
    }
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
