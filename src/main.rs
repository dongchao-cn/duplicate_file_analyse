use clap::Parser;
use walkdir::WalkDir;
use std::path::PathBuf;
use hex;
use sha3::{Digest, Sha3_256};
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Parser)]
struct Args {
    #[arg(short = 'p', value_name = "path")]
    path: String
}

fn main() {
    println!("Hello, world!");
    let args = Args::parse();

    println!("args.path: {}", args.path);

    // let all_files = get_all_files(&args.path);
    // for each in all_files {
    //     println!("{}", each.display());
    // }

    calc_hash("");
}

fn get_all_files(path: &str) -> Vec<PathBuf> {
    let mut result = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let p = entry.path().to_path_buf();
            // println!("{}", p.display());
            result.push(p);
        }
    }
    result
}

const READ_BLOCK_SIZE: usize = 4096;

fn calc_hash(file_name: &str) -> String {
    // create a SHA3-256 object
    let mut hasher = Sha3_256::new();

    let f = File::open(file_name).unwrap();
    let mut reader = BufReader::new(f);
    let mut buffer = [0; READ_BLOCK_SIZE];

    loop {
        let bytes_read = reader.read(&mut buffer).unwrap();

        if bytes_read == 0 {
            break;
        }

        hasher.update(buffer);
    }

    // read hash digest
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
